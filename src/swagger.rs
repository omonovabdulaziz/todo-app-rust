use utoipa::OpenApi;
use crate::db::model::Todo;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todos::handle_get_todos,
        crate::routes::todos::handle_create_todo,
    ),
    components(schemas(Todo)),
    tags(
        (name = "todo", description = "Todo management endpoints")
    )
)]
pub struct ApiDoc;
