use warp::{Filter, Rejection, Reply};
use sqlx::PgPool;
use crate::db::queries::{get_all_todos, create_todo};
use crate::db::models::Todo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

pub fn todo_routes(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.clone());

    // GET /todos
    let get = warp::path("todos")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handle_get_todos);

    // POST /todos
    let create = warp::path("todos")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(handle_create_todo);

    get.or(create)
}

async fn handle_get_todos(pool: PgPool) -> Result<impl Reply, Rejection> {
    match get_all_todos(&pool).await {
        Ok(todos) => Ok(warp::reply::json(&todos)),
        Err(_) => Err(warp::reject::custom("Failed to fetch todos")),
    }
}

async fn handle_create_todo(body: CreateTodoRequest, pool: PgPool) -> Result<impl Reply, Rejection> {
    match create_todo(&pool, &body.title).await {
        Ok(todo) => Ok(warp::reply::json(&todo)),
        Err(_) => Err(warp::reject::custom("Failed to create todo")),
    }
}
