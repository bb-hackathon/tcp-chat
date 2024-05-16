use crate::proto;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct AuthToken {
    token: u128,
}

impl AuthToken {
    pub fn new(random: &mut ChaCha20Rng) -> Self {
        let mut u128_pool = [0u8; 16];
        random.fill_bytes(&mut u128_pool);
        let token = u128::from_le_bytes(u128_pool);
        Self { token }
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
        Self::from_str(&proto_token.token)
    }
}

impl FromStr for AuthToken {
    type Err = ParseIntError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            token: u128::from_str_radix(str, 16)?,
        })
    }
}

impl fmt::Display for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>032x}", self.token)?;
        Ok(())
    }
}

impl fmt::Display for proto::AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>032}", self.token)?;
        Ok(())
    }
}

impl FromStr for proto::AuthToken {
    type Err = ParseIntError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        AuthToken::from_str(token).map(|t| t.into())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::AuthToken;
    use crate::proto;

    #[test]
    fn conversion_roundtrip() {
        const INNER: u128 = 123;
        let token = AuthToken { token: INNER };
        let proto_token = proto::AuthToken {
            token: format!("{:>032x}", INNER),
        };

        let converted_to_proto: proto::AuthToken = token.clone().into();
        assert_eq!(converted_to_proto, proto_token);

        let converted_from_proto: AuthToken = proto_token.clone().try_into().unwrap();
        assert_eq!(token, converted_from_proto);

        assert_eq!(token.to_string(), proto_token.to_string());
    }
}
