use crate::frame::frame_errors::ParseError;
use crate::frame::types;

// Implements Authenticate message.
#[derive(Debug)]
pub struct Authenticate {
    pub authenticator_name: String,
}

impl Authenticate {
    pub fn deserialize(buf: &mut &[u8]) -> Result<Self, ParseError> {
        let authenticator_name = types::read_string(buf)?.to_string();

        Ok(Authenticate { authenticator_name })
    }
}

#[derive(Debug)]
pub struct AuthSuccess {
    pub success_message: Option<Vec<u8>>,
}

impl AuthSuccess {
    pub fn deserialize(buf: &mut &[u8]) -> Result<Self, ParseError> {
        let success_message = types::read_bytes_opt(buf)?.map(|b| b.to_owned());

        Ok(AuthSuccess { success_message })
    }
}

#[derive(Debug)]
pub struct AuthChallenge {
    pub authenticate_message: Option<Vec<u8>>,
}

impl AuthChallenge {
    pub fn deserialize(buf: &mut &[u8]) -> Result<Self, ParseError> {
        let authenticate_message = types::read_bytes_opt(buf)?.map(|b| b.to_owned());

        Ok(AuthChallenge {
            authenticate_message,
        })
    }
}
