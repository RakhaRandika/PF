use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FaceBox {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct DetectionResponse {
    pub faces: Vec<FaceBox>,
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub num_faces: usize,
}

#[derive(Debug, Serialize)]
pub struct ProcessResponse {
    pub image: String,
    pub num_faces: usize,
    pub faces: Vec<FaceBox>,
    pub processing_time_ms: u64,
}
