use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


use crate::configs::open_api::OpenApiDoc;


pub fn swagger_routes() -> Router<crate::state::AppState> {
    Router::new()
      .merge(SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", OpenApiDoc::openapi()))
      
}