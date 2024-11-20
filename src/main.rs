use warp::Filter;
mod config;
mod db;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().ok();

    let pool = db::connect().await?;

    let routes = routes::todos::todo_routes(pool);

    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
