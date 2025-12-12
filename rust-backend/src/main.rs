mod blur;
mod models;

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use crate::blur::process_image_with_blur;
use crate::models::{DetectionResponse, ProcessResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/process", post(process_image))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Rust backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Face Blur Backend API"
}


async fn process_image(mut multipart: Multipart) -> Result<Json<ProcessResponse>, StatusCode> {
    let mut image_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "image" {
            let data = field.bytes().await.unwrap();
            image_data = Some(data.to_vec());
        }
    }

    let image_data = image_data.ok_or(StatusCode::BAD_REQUEST)?;

    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .part(
            "image",
            reqwest::multipart::Part::bytes(image_data)
                .file_name("image.jpg")
                .mime_str("image/jpeg")
                .unwrap(),
        );

    let detection_result = client
        .post("http://localhost:5000/detect")
        .multipart(form)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !detection_result.status().is_success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let detection: DetectionResponse = detection_result
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    use base64::{Engine as _, engine::general_purpose};
    let img_bytes = general_purpose::STANDARD.decode(&detection.image)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blurred_image = process_image_with_blur(&img_bytes, &detection.faces)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result_base64 = general_purpose::STANDARD.encode(&blurred_image);

    Ok(Json(ProcessResponse {
        image: result_base64,
        num_faces: detection.num_faces,
        faces: detection.faces,
        processing_time_ms: 0,
    }))
}
