use std::task;

use sqlx::postgres::Postgres;
use sqlx::PgPool;
use sqlx::{Row, Transaction};

pub async fn queue_worker(id: u32, pool: PgPool) {
    loop {}
}

async fn get_task(pool: &PgPool) -> Option<(i32, i32)> {
    let mut tx: Transaction<'_, Postgres> =
        pool.begin().await.expect("Failed to begin transaction");

    let row = sqlx::query!(
        r#"SELECT id, video_id
           FROM queue
           WHERE task_status = 'pending'
           ORDER BY priority DESC
           FOR UPDATE SKIP LOCKED
           LIMIT 1"#,
    )
    .fetch_optional(&mut *tx)
    .await
    .expect("Failed to fetch task");

    if let Some(row) = row {
        let task_id: i32 = row.id;
        let video_id: i32 = row.video_id;

        sqlx::query!(
            "UPDATE queue SET task_status = 'in_progress' WHERE id = $1",
            task_id
        )
        .execute(&mut *tx)
        .await
        .expect("Failed to update task status");

        tx.commit().await.expect("Failed to commit transaction");

        Some((task_id, video_id))
    } else {
        None
    }
}

async fn mark_task_completed(pool: &PgPool, task_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE queue SET task_status = 'completed' WHERE id = $1", task_id)
        .execute(pool)
        .await?;

    Ok(())
}
