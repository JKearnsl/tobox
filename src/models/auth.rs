

// SignIn model

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
}
