use utoipa::OpenApi;

use crate::handlers::{auth_handler, user_basket_handler, user_transaction_handler, basket_transaction_handle};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        auth_handler::register,
        auth_handler::login,
        // Baskets
        user_basket_handler::create_basket,
        user_basket_handler::get_all_baskets,
        user_basket_handler::update_basket,
        // Transactions
        user_transaction_handler::create_transaction,
        basket_transaction_handle::get_basket_transactions,
        
    ),
    components(
        schemas(
            crate::dtos::common_dto::CommonErrorResponse,
            crate::dtos::basket_dto::BasketResponse,
            crate::dtos::basket_dto::BasketCategoryResponse,
            crate::dtos::basket_dto::CreateBasketPayload,
            crate::dtos::basket_dto::UpdateBasketPayload,
            crate::dtos::transaction_dto::TransactionResponse,
            crate::dtos::transaction_dto::TransactionWithDetails,
            crate::dtos::transaction_dto::TransactionTypeResponse,
            crate::dtos::transaction_dto::CreateTransactionPayload,
            crate::dtos::transaction_dto::CreateTransactionTypePayload,
            crate::dtos::transaction_dto::UpdateTransactionTypePayload,
            crate::dtos::transaction_dto::TransactionBasketInfo,
            crate::dtos::transaction_dto::TransactionTypeInfo,
            crate::dtos::transaction_dto::TransactionDetailResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication management"),
        (name = "baskets", description = "Basket management for storing money"),
        (name = "transactions", description = "Transaction management"),
        (name = "admin", description = "Admin operations")
    ),
    info(
        title = "Kakeiboku API",
        version = "0.0.1",
        description = "API documentation for Kakeiboku backend. This API provides endpoints for user management, basket management, and transaction functionalities.",
        contact(
            name = "Dewa",
            url = "https://github.com/DewaSRY",
            email = "sdewa6645@gmail.com"
        ),
        license(
            name = "MIT OR Apache-2.0",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&SecurityAddon)
)]
pub struct OpenApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}
