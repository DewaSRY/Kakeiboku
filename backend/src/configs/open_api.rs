use utoipa::OpenApi;

use crate::handlers::{auth_handler, basket_handler, transaction_handler};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        auth_handler::register,
        auth_handler::login,
        // Baskets
        basket_handler::create_basket,
        basket_handler::get_all_baskets,
        basket_handler::get_main_basket,
        basket_handler::get_branch_baskets,
        basket_handler::get_basket_by_id,
        basket_handler::update_basket,
        basket_handler::delete_basket,
        basket_handler::deposit_to_main_basket,
        basket_handler::transfer_to_branch,
        basket_handler::create_basket_category,
        basket_handler::get_all_basket_categories,
        basket_handler::delete_basket_category,
        // Transactions
        transaction_handler::create_transaction,
        transaction_handler::get_transaction_by_id,
        transaction_handler::get_user_transactions,
        transaction_handler::get_basket_transactions,
        transaction_handler::create_transaction_type,
        transaction_handler::get_all_transaction_types,
        transaction_handler::get_flat_transaction_types,
        transaction_handler::update_transaction_type,
        transaction_handler::delete_transaction_type,
    ),
    components(
        schemas(
            crate::models::users::User,
            crate::models::baskets::Basket,
            crate::models::baskets_category::BasketCategory,
            crate::models::transactions::Transaction,
            crate::models::transactions_type::TransactionType,
            crate::models::transactions_detail::TransactionDetail,
            crate::dtos::basket_dto::BasketResponse,
            crate::dtos::basket_dto::BasketCategoryResponse,
            crate::dtos::basket_dto::CreateBasketPayload,
            crate::dtos::basket_dto::UpdateBasketPayload,
            crate::dtos::basket_dto::DepositToMainBasketPayload,
            crate::dtos::basket_dto::TransferToBranchPayload,
            crate::dtos::transaction_dto::TransactionResponse,
            crate::dtos::transaction_dto::TransactionWithDetails,
            crate::dtos::transaction_dto::TransactionTypeResponse,
            crate::dtos::transaction_dto::CreateTransactionPayload,
            crate::dtos::transaction_dto::CreateTransactionTypePayload,
            crate::dtos::transaction_dto::UpdateTransactionTypePayload,
            crate::dtos::transaction_dto::TransactionBasketInfo,
            crate::dtos::transaction_dto::TransactionTypeInfo,
            crate::dtos::transaction_dto::TransactionDetailResponse,
            crate::dtos::common_dto::CommonResponse,
            crate::dtos::common_dto::CommonErrorResponse,
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
