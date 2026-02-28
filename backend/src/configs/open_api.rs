use utoipa::OpenApi;


use crate::handlers::user_handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handlers::list_users,
        user_handlers::create_user,
    ),
    components(
        schemas(
            crate::models::user::User
        )
    ),
    tags(
        (name = "users", description = "User management")
    ),
    info(
        title = "Kakeiboku API",
        version = "0.0.1",
        description = "API documentation for Kakeiboku backend. This API provides endpoints for user management and other functionalities related to the Kakeiboku application.",
        contact(
            name = "Dewa",
            url = "https://github.com/DewaSRY",
            email = "sdewa6645@gmail.com"
        ),
        license(
            name = "MIT OR Apache-2.0",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
pub struct OpenApiDoc;