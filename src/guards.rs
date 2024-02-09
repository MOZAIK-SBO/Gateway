use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct JwtToken(String);

#[derive(Debug)]
pub enum JwtTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = JwtTokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = req.headers().get_one("authorization") {
            let auth_header_parts = auth_header.split_whitespace().collect::<Vec<_>>();

            if auth_header_parts.len() == 2
                && auth_header_parts[0].to_string().to_lowercase() == "bearer"
            {
                Outcome::Success(JwtToken(auth_header_parts[1].into()))
            } else {
                Outcome::Error((Status::Unauthorized, JwtTokenError::Invalid))
            }
        } else {
            Outcome::Error((Status::Unauthorized, JwtTokenError::Missing))
        }
    }
}
