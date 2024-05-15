use crate::proto;
use rand_chacha::ChaChaRng;
use rand_core::RngCore;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct AuthToken {
    token: u128,
}

impl AuthToken {
    pub fn new(random: &mut ChaChaRng) -> Self {
        let mut u128_pool = [0u8; 16];
        random.fill_bytes(&mut u128_pool);
        let token = u128::from_le_bytes(u128_pool);
        Self { token }
    }
}

impl fmt::Display for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>032x}", self.token)?;
        Ok(())
    }
}

impl From<AuthToken> for proto::AuthToken {
    fn from(auth_token: AuthToken) -> Self {
        Self {
            token: auth_token.to_string(),
        }
    }
}

impl TryFrom<proto::AuthToken> for AuthToken {
    type Error = std::num::ParseIntError;

    fn try_from(proto_token: proto::AuthToken) -> Result<Self, Self::Error> {
        Ok(Self {
            token: u128::from_str_radix(&proto_token.token, 16)?,
        })
    }
}
