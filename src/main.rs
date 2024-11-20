use sqlx::{PgPool, postgres::PgPoolOptions};
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "postgres://username:password@localhost/todo_app";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let pool_filter = warp::any().map(move || pool.clone());

    let routes = warp::path("todos")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(get_todos)
        .or(warp::path("todos")
            .and(warp::post())
            .and(warp::body::json())
            .and(pool_filter.clone())
            .and_then(create_todo));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
