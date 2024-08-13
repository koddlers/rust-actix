use crate::utils::api_response::ApiResponse;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::AUTHORIZATION;
use actix_web::Error;
use actix_web_lab::middleware::Next;

pub async fn check_auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = request.headers().get(AUTHORIZATION);
    if auth.is_none() {
        return Err(Error::from(ApiResponse::new(401, String::from("Unauthorized"))));
    }

    // TODO: this panics. fix this
    // let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    // let _claim = decode_jwt(token).unwrap();

    next.call(request).await.map_err(
        |err| Error::from(ApiResponse::new(500, err.to_string()))
    )
}