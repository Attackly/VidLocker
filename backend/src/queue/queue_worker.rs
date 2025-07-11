use crate::func::video::download_video_simple_ydl;
use sqlx::PgPool;
use sqlx::Transaction;
use sqlx::postgres::Postgres;
use tracing::{debug, info};

pub async fn queue_worker(id: u32, pool: PgPool, interval: u64 ) {
    let mut poll = tokio::time::interval(std::time::Duration::from_secs(interval));
    loop {

        poll.tick().await;

        match get_task(&pool).await {
            Some((0, s, _x)) if s == "0" => {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Some((task_id, url, path)) => {
                info!("Worker {id} Found a Task. Will download. task with id: {task_id}");
                download_video_simple_ydl(url,path).await;

                mark_task_completed(&pool, task_id)
                    .await
                    .expect("Worker {id} Failed to mark task as completed");
            }
            None => {
                debug!("Worker {} is idle", id);
            }
        }
    }
}

async fn get_task(pool: &PgPool) -> Option<(i32, String, String)> {
    let mut tx: Transaction<'_, Postgres> =
        pool.begin().await.expect("Failed to begin transaction");

    // TODO Lookup if the video has been downloaded already.


    let row = sqlx::query!(
        r#"SELECT
            queue.id AS queue_id,
            queue.video_id,
            queue.priority,
            videos.url,
            videos.viewkey,
            videos.path
        FROM
            queue
        JOIN
            videos
        ON
            queue.video_id = videos.id
           WHERE queue.task_status = 'pending'
           ORDER BY priority DESC
           FOR UPDATE SKIP LOCKED
           LIMIT 1"#,
    )
    .fetch_optional(&mut *tx)
    .await
    .expect("Failed to fetch task");

    if row.is_none() {
        info!("No tasks found in the queue.");
        return None;
    }

    let row = row.unwrap();

    let viewkey: String = row.viewkey.unwrap();
    let already_downloaded = sqlx::query!("SELECT Id, viewkey FROM videos WHERE viewkey = $1", &viewkey)
        .fetch_optional(&mut *tx)
        .await;
    if already_downloaded.unwrap().is_none() {
        info!("Video has already been downloaded.");

        // Handle if it has been downloaded already
        //
        return Some((0, "0".to_string(), "".to_string()));
    }

    let task_id: i32 = row.queue_id;
    let url: String = row.url;
    let path: String = row.path;

    sqlx::query!(
        "UPDATE queue SET task_status = 'in_progress' WHERE id = $1",
        task_id
    )
    .execute(&mut *tx)
    .await
    .expect("Failed to update task status");

    tx.commit().await.expect("Failed to commit transaction");

    Some((task_id, url, path))
}

async fn mark_task_completed(pool: &PgPool, task_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM queue WHERE id = $1", task_id)
        .execute(pool)
        .await
        .expect("Failed to mark task as completed");

    // Remove the entry from the queue. And Complete videos entry
    Ok(())
}
