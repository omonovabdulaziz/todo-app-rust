use crate::db::queries::{create_todo, get_all_todos};
use sqlx::PgPool;
use warp::{Filter, Rejection, Reply};
use warp::reject::Reject;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}
#[derive(Debug)]
struct CustomError(String);

impl Reject for CustomError {}

pub fn todo_routes(pool: PgPool) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.clone());

    let get = warp::path("todos")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handle_get_todos);

    let create = warp::path("todos")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(handle_create_todo);

    get.or(create)
}

pub async fn handle_get_todos(pool: PgPool) -> Result<impl Reply, Rejection> {
    match get_all_todos(&pool).await {
        Ok(todos) => Ok(warp::reply::json(&todos)),
        Err(_) => Err(warp::reject::custom(CustomError(
            "Failed to fetch todos".to_string(),
        ))),
    }
}

pub async fn handle_create_todo(
    body: CreateTodoRequest,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    match create_todo(&pool, &body.title).await {
        Ok(todo) => Ok(warp::reply::json(&todo)),
        Err(_) => Err(warp::reject::custom(CustomError(
            "Failed to create todo".to_string(),
        ))),
    }
}