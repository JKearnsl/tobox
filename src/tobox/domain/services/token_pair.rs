use crate::domain::models::token_pair::TokenPair;
use crate::domain::models::user::UserId;

pub struct TokenPairService {}

impl TokenPairService {
    pub fn create_pair(
        &self,
        public: String,
        enc_private: String,
        user_id: UserId,
    ) -> TokenPair {
        TokenPair {
            public,
            enc_private,
            user_id,
        }
    }

    pub fn update_pair(
        &self,
        pair: TokenPair,
        new_public: String,
        new_enc_private: String,
    ) -> TokenPair {
        TokenPair {
            public: new_public,
            enc_private: new_enc_private,
            ..pair
        }
    }
}
