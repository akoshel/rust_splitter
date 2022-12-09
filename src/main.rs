use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use serde::{Deserialize, Serialize};
mod api;
mod splitter_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_settings = splitter_config::ExperimentConfig::new().unwrap();
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(splitter_config::ExperimentConfig::new().unwrap()))
            .service(api::post_request::index)
            .service(api::post_request::make_request)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

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
        let app = test::init_service(App::new().service(api::post_request::index)).await;
        let resp = test::call_service(&app, req).await;
        // let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_request_post() {
        #[derive(Serialize, Deserialize)]
        struct TestJsonData {
            user_id: String,
            value: String, // {"login":"my_login","password":"my_password"}
        }
        let test_json1 = TestJsonData {
            user_id: "123".to_string(),
            value: "android".to_string(),
        }; 
        let test_json2 = TestJsonData {
            user_id: "123".to_string(),
            value: "ios".to_string(),
        };
        let app = test::init_service(
            App::new()
                .app_data(Data::new(splitter_config::ExperimentConfig::new().unwrap()))
                .service(api::post_request::make_request),
        )
        .await;
        let req1 = test::TestRequest::post()
            .uri("/request")
            .set_json(test_json1)
            .to_request();
        let req2 = test::TestRequest::post()
            .uri("/request")
            .set_json(test_json2)
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        let resp2 = test::call_service(&app, req2).await;
        assert!(resp1.status().is_success());
        assert!(resp2.status().is_server_error());
    }
}
