use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;

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