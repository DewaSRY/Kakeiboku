use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
}