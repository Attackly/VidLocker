use sqlx::postgres::Postgres;
use sqlx::PgPool;
use sqlx::Transaction;

use crate::func::video::download_video_simple_ydl;

pub async fn queue_worker(id: u32, pool: PgPool) {
    loop {
        match get_task(&pool).await {
            Some((task_id, url)) => {
                download_video_simple_ydl(url);

                mark_task_completed(&pool, task_id)
                    .await
                    .expect("Failed to mark task as completed");
            }
            None => {
                println!("Worker {} is idle", id);
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        }
    }
}
async fn get_task(pool: &PgPool) -> Option<(i32, String)> {
    let mut tx: Transaction<'_, Postgres> =
        pool.begin().await.expect("Failed to begin transaction");

    let row = sqlx::query!(
        r#"SELECT
            queue.id AS queue_id,
            queue.video_id,
            queue.priority,
            videos.url
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

    if let Some(row) = row {
        let task_id: i32 = row.queue_id;
        let url: String = row.url;

        sqlx::query!(
            "UPDATE queue SET task_status = 'in_progress' WHERE id = $1",
            task_id
        )
        .execute(&mut *tx)
        .await
        .expect("Failed to update task status");

        tx.commit().await.expect("Failed to commit transaction");

        Some((task_id, url))
    } else {
        None
    }
}

async fn mark_task_completed(pool: &PgPool, task_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE queue SET task_status = 'completed' WHERE id = $1",
        task_id
    )
    .execute(pool)
    .await
    .expect("Failed to mark task as completed");

    // Remove the entry from the queue. And Complete videos entry
    Ok(())
}
