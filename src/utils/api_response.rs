use std::fmt::{Debug, Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;

#[derive(Debug)]
pub struct ApiResponse {
    body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        Self {
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        println!("request: {:?}", req);
        let host = req.headers().get("host").unwrap().to_owned();
        println!("host: {:?}", host);

        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}


impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} \n Status Code: {}", self.body, self.status_code())
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}