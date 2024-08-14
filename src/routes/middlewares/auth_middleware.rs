use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::decode_jwt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use actix_web_lab::middleware::Next;

pub async fn check_auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = request.headers().get(AUTHORIZATION);
    if auth.is_none() {
        return Err(Error::from(ApiResponse::new(401, String::from("Unauthorized"))));
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    // print!("{token}");
    let claim = decode_jwt(token);
    match claim {
        Ok(_claim) => {
            // print!("Claim: {:?}", _claim);
            request.extensions_mut().insert(_claim.claims);
        }
        Err(err) => println!("Error in claim: {err:?}"),
    }


    next.call(request).await.map_err(
        |err| Error::from(ApiResponse::new(500, err.to_string()))
    )
}