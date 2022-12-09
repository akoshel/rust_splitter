use std::env::split_paths;

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder, error::ResponseError
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::splitter_config;

#[derive(Debug, Display, Error)]
pub enum RequestError {
    TokenezationError,
    PredictError,
}

impl ResponseError for RequestError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskRequest {
    user_id: String,
    value: String,
}

#[post("/request")]
pub async fn make_request(
    selector_state: Data<splitter_config::ExperimentConfig>,
    request: Json<SubmitTaskRequest>,
) -> Result<Json<SubmitTaskRequest>, RequestError> {
    println!("{:?}", request);
    // let is_in_experiment = selector_state.selector.validate(&request.value);
    let experiment_input = splitter_config::ExperimentInput::new(&request.user_id, &request.value);
    match selector_state.selector.validate(&experiment_input) {
        true => Ok(request),
        _ => Err(RequestError::PredictError),
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test, App,
    };
    #[actix_web::test]
    async fn test_index_ok() {
        // let req = test::TestRequest::default()
        //     .insert_header(ContentType::plaintext())
        //     .to_http_request();
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let app = test::init_service(App::new().service(index)).await;
        let resp = test::call_service(&app, req).await;
        // let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
