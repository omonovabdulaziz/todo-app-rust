use sqlx::PgPool;
use crate::db::model::Todo;

pub async fn get_all_todos(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos"
    )
        .fetch_all(pool)
        .await
}

pub async fn create_todo(pool: &PgPool, title: &str) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed",
        title
    )
        .fetch_one(pool)
        .await
}
