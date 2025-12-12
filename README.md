 # Face Blur Detection - Sistem Perlindungan Privasi
_Pendekatan Pemrograman Fungsional dengan Rust_
**Penulis:** Rayhan Marcello Ananda Purnomo | Maulida Rahmayanti | Muhammad Rakha Randhika | Faqih Chairul Anam | Amisha Nabila Wiguna | Nurhafid Sudarianto
---

## Abstrak
Face Blur Detection adalah aplikasi perlindungan privasi yang secara otomatis mendeteksi dan mengaburkan wajah pada gambar, membantu menjaga data visual tetap aman. Proyek ini memperlihatkan pipeline deteksi wajah dan blur yang dibangun dengan backend Rust, layanan detektor Python, dan frontend React. Tujuannya adalah menyediakan pipeline pemrosesan gambar yang handal dan berkinerja tinggi dengan menerapkan prinsip pemrograman fungsional di Rust (fungsi murni, imutabilitas, dan komposisi). Teknologi utama meliputi Rust (`axum`, `tokio`), `rayon` untuk paralelisme, dan detektor YOLO di Python.

---

## Pendahuluan

Aplikasi ini dibuat untuk secara otomatis mendeteksi wajah pada gambar dan menerapkan blur pada area wajah sebagai langkah perlindungan privasi. Dengan mengaburkan wajah sebelum gambar dibagikan atau disimpan, aplikasi membantu mengurangi risiko pelanggaran privasi dan memastikan data visual lebih aman untuk distribusi.

Rust dipilih sebagai backend karena kemampuannya memberikan performa tinggi sekaligus jaminan keamanan memori melalui mekanisme ownership dan borrow checker. Kombinasi ini cocok untuk pemrosesan gambar yang intensif CPU dan kebutuhan concurrency yang aman; runtime asinkron (`tokio`) dan framework `axum` mendukung arsitektur layanan yang responsif.

Prinsip pemrograman fungsional diterapkan untuk meningkatkan keterbacaan, testabilitas, dan prediktabilitas kode. Pendekatan seperti fungsi murni, imutabilitas, dan komposisi memudahkan pemisahan langkah-langkah pipeline (decoding → deteksi → blur → encoding) menjadi fungsi-fungsi kecil yang dapat diuji secara terpisah dan diparalelisasi dengan `rayon` ketika sesuai.

Yang membuat solusi ini menarik adalah pemisahan tanggung jawab antara deteksi dan pemrosesan: model YOLO dan logika ML diletakkan di layanan Python, sementara Rust menangani pemrosesan gambar terprediksi dan berkinerja tinggi. Desain ini memungkinkan fungsi-fungsi Rust dibuat kecil, mudah diuji, dan dapat disusun (composable), sehingga mempercepat pengembangan fitur tambahan dan mempermudah pemeliharaan.

---

## Latar Belakang dan Konsep

proyek ini bertujuan membangun pipeline deteksi wajah dan pengaburan (blur) yang dapat diandalkan, cepat, dan mudah dipelihara. Perancangan memisahkan tugas deteksi (ML) dan tugas pemrosesan gambar sehingga masing‑masing komponen dapat memakai alat dan bahasa yang paling sesuai: model YOLO dan logika pembelajaran mesin di Python, sedangkan pemrosesan gambar berkinerja tinggi dan API diletakkan di Rust.

Struktur repositori mencerminkan pemisahan tersebut:
- `python-detector/` — layanan deteksi (Ultralytics YOLO) yang menerima gambar, menghasilkan bounding box, dan mengembalikan metadata/image (JSON/base64).
- `rust-backend/` — backend API (Axum + Tokio) yang menerima upload, meneruskan ke detektor, lalu memproses area wajah menggunakan crate `image` dan `rayon`.
- `react-frontend/` — antarmuka pengguna untuk mengunggah gambar, menampilkan preview, dan menerima hasil blur.

Konsep kunci dan peran teknologi:

| Teknologi | Peran / Keterangan | Lokasi / Catatan |
|---|---|---|
| Rust | Bahasa utama untuk backend, pemrosesan gambar berkinerja tinggi, dan concurrency | `rust-backend/` |
| Axum | Framework HTTP minimalis untuk routing, extractor (Multipart) dan integrasi async | `rust-backend/src` |
| Tokio | Runtime asinkron untuk menjalankan server dan I/O non-blok | dependency di `rust-backend/` |
| Rayon | Data-parallelism untuk menjalankan operasi intensif (blur pixel) secara paralel | dipakai di `rust-backend/src/blur.rs` |
| image | Manipulasi gambar (load, crop, edit, encode/write) | crate di Rust |
| Reqwest | HTTP client di Rust untuk memanggil layanan detektor Python | dipakai di `rust-backend/src/main.rs` |
| Serde / serde_json | (De)serialisasi JSON untuk pertukaran metadata (bounding boxes, dll.) | digunakan antar layanan |
| Base64 | Encoding/decoding gambar saat mengirim/ menerima image dalam payload JSON | digunakan pada komunikasi antara layanan |
| Tower-http / CORS | Middleware untuk konfigurasi CORS dan utilitas HTTP pada stack Axum | middleware di `rust-backend/` |
| Tracing / tracing_subscriber | Observability / logging untuk tracing request dan performa | konfigurasi logging di Rust |
| Python + Ultralytics YOLO | Ekosistem ML untuk deteksi wajah/objek; model `.pt` dikelola dan dijalankan di sini | `python-detector/` (Flask/FastAPI) |
| React | Frontend untuk upload, preview, dan menampilkan hasil; biasanya menggunakan `axios` untuk HTTP | `react-frontend/` |

Alur data singkat:
1. Pengguna mengunggah gambar dari frontend ke backend (`/process`).
2. Backend mengirim gambar ke `python-detector` untuk deteksi wajah.
3. `python-detector` mengembalikan bounding box dan (opsional) image base64.
4. Backend memproses area wajah (crop → blur) secara paralel menggunakan `rayon`, lalu mengembalikan hasil (base64) ke frontend.

Manfaat desain ini: pemisahan tanggung jawab (ML vs pemrosesan), kemampuan untuk mengoptimalkan performance-critical code di Rust, dan kemudahan iterasi model di Python. Pendekatan fungsional di Rust (fungsi kecil, komposisi, imutabilitas lokal) meningkatkan testabilitas dan prediktabilitas pemrosesan gambar.

---

## Kode Sumber dan Penjelasan
Struktur folder proyek:

```
PF/
├─ best (3).pt
├─ best (4).pt
├─ python-detector/
│  ├─ app.py
│  ├─ requirements.txt
│  └─ test_model.py
├─ rust-backend/
│  ├─ Cargo.toml
│  └─ src/
│     ├─ main.rs
│     ├─ blur.rs
│     └─ models.rs
├─ react-frontend/
│  ├─ package.json
│  ├─ public/
│  │  └─ index.html
│  └─ src/
│     ├─ App.js
│     ├─ index.js
│     ├─ index.css
│     └─ App.css
├─ README.md
└─ start-all.bat
```

Source Code:

**`rust-backend/Cargo.toml`**
```toml
[package]
name = "face-blur-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["multipart"] }
tokio = { version = "1.35", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "fs"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
image = "0.24"
rayon = "1.8"
base64 = "0.21"
reqwest = { version = "0.11", features = ["json", "multipart"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
```

**`rust-backend/src/main.rs`**
```rust
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
        .route("/health", get(health))
        .route("/process", post(process_image))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Rust backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Face Blur Backend API"
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "face-blur-backend"
    }))
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
```

**`rust-backend/src/blur.rs`**
```rust
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::io::Cursor;

use crate::models::FaceBox;

pub fn process_image_with_blur(
    image_data: &[u8],
    faces: &[FaceBox],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(image_data)?;
    let mut img = img.to_rgba8();

    let blur_regions: Vec<_> = faces
        .par_iter()
        .map(|face| {
            let x1 = face.x1.max(0) as u32;
            let y1 = face.y1.max(0) as u32;
            let x2 = face.x2.min(img.width() as i32) as u32;
            let y2 = face.y2.min(img.height() as i32) as u32;

            if x2 <= x1 || y2 <= y1 {
                return None;
            }

            let face_region = img.view(x1, y1, x2 - x1, y2 - y1).to_image();

            let blurred = apply_gaussian_blur(&face_region, 25.0);

            Some((x1, y1, blurred))
        })
        .collect();

    for region in blur_regions.into_iter().flatten() {
        let (x, y, blurred) = region;
        
        for (dx, dy, pixel) in blurred.enumerate_pixels() {
            img.put_pixel(x + dx, y + dy, *pixel);
        }
    }

    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    dynamic_img.write_to(
        &mut std::io::BufWriter::new(cursor),
        image::ImageOutputFormat::Jpeg(90),
    )?;

    Ok(output)
}

fn apply_gaussian_blur(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    sigma: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    
    let kernel_size = ((sigma * 6.0).ceil() as usize) | 1;
    let kernel = create_gaussian_kernel(kernel_size, sigma);
    let half_kernel = kernel_size / 2;

    let pixels: Vec<_> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width)
                .map(|x| {
                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    let mut a = 0.0;
                    let mut weight_sum = 0.0;

                    for ky in 0..kernel_size {
                        for kx in 0..kernel_size {
                            let px = (x as i32 + kx as i32 - half_kernel as i32)
                                .max(0)
                                .min(width as i32 - 1) as u32;
                            let py = (y as i32 + ky as i32 - half_kernel as i32)
                                .max(0)
                                .min(height as i32 - 1) as u32;

                            let pixel = img.get_pixel(px, py);
                            let weight = kernel[ky][kx];

                            r += pixel[0] as f32 * weight;
                            g += pixel[1] as f32 * weight;
                            b += pixel[2] as f32 * weight;
                            a += pixel[3] as f32 * weight;
                            weight_sum += weight;
                        }
                    }

                    Rgba([
                        (r / weight_sum) as u8,
                        (g / weight_sum) as u8,
                        (b / weight_sum) as u8,
                        (a / weight_sum) as u8,
                    ])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    ImageBuffer::from_vec(width, height, pixels.into_iter().flat_map(|p| p.0).collect())
        .unwrap()
}

fn create_gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let center = size / 2;
    let mut sum = 0.0;

    for i in 0..size {
        for j in 0..size {
            let x = i as f32 - center as f32;
            let y = j as f32 - center as f32;
            let value = (-(x * x + y * y) / (2.0 * sigma * sigma)).exp();
            kernel[i][j] = value;
            sum += value;
        }
    }

    for i in 0..size {
        for j in 0..size {
            kernel[i][j] /= sum;
        }
    }

    kernel
}
```

**`rust-backend/src/models.rs`**
```rust
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
```

**`python-detector/app.py`**
```python
from flask import Flask, request, jsonify
from flask_cors import CORS
import cv2
import numpy as np
from ultralytics import YOLO
import base64
from io import BytesIO
from PIL import Image
import os

app = Flask(__name__)
CORS(app)

MODEL_PATH = os.path.join(os.path.dirname(__file__), '..', 'best (4).pt')
model = YOLO(MODEL_PATH)

@app.route('/health', methods=['GET'])
def health():
    return jsonify({"status": "healthy", "service": "face-detection"})

@app.route('/detect', methods=['POST'])
def detect_faces():
    try:
        if 'image' not in request.files:
            return jsonify({"error": "No image provided"}), 400
        
        file = request.files['image']

        image_bytes = file.read()
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        
        if img is None:
            return jsonify({"error": "Invalid image"}), 400
        height, width = img.shape[:2]

        print(f"Image shape: {img.shape}, dtype: {img.dtype}")
        results = model(img, conf=0.15, iou=0.45, max_det=50, imgsz=640, verbose=True)
        
        faces = []
        print(f"Number of results: {len(results)}")
        for result in results:
            boxes = result.boxes
            print(f"Number of boxes: {len(boxes)}")
            for box in boxes:
                x1, y1, x2, y2 = box.xyxy[0].cpu().numpy()
                confidence = float(box.conf[0])
                
                print(f"Detected face: ({x1}, {y1}, {x2}, {y2}) confidence: {confidence}")
                
                faces.append({
                    "x1": int(x1),
                    "y1": int(y1),
                    "x2": int(x2),
                    "y2": int(y2),
                    "confidence": confidence
                })
        
        print(f"Total faces detected: {len(faces)}")
        
        _, buffer = cv2.imencode('.jpg', img)
        img_base64 = base64.b64encode(buffer).decode('utf-8')
        
        return jsonify({
            "faces": faces,
            "image": img_base64,
            "width": width,
            "height": height,
            "num_faces": len(faces)
        })
    
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    print("Starting Face Detection Service...")
    print(f"Model loaded from: {MODEL_PATH}")
    app.run(host='0.0.0.0', port=5000, debug=True)
```

**`python-detector/requirements.txt`**
```text
flask>=3.0.0
flask-cors>=4.0.0
opencv-python>=4.8.0
numpy>=1.24.0
ultralytics>=8.0.0
Pillow>=10.0.0
torch>=2.0.0
torchvision>=0.15.0
```

**`python-detector/test_model.py`**
```python
from ultralytics import YOLO
import sys

model = YOLO('../best (3).pt')

print("=" * 50)
print("MODEL INFORMATION")
print("=" * 50)
print(f"Model type: {type(model)}")
print(f"Model task: {model.task if hasattr(model, 'task') else 'Unknown'}")

if hasattr(model, 'names'):
    print(f"\nClass names: {model.names}")
    print(f"Number of classes: {len(model.names)}")
else:
    print("\nNo class names found")

print("\nModel info:")
try:
    info = model.info(verbose=True)
except Exception as e:
    print(f"Error getting info: {e}")

print("\n" + "=" * 50)
print("Testing with a sample prediction...")
print("=" * 50)

try:
    print("\nModel attributes:")
    for attr in dir(model):
        if not attr.startswith('_'):
            try:
                value = getattr(model, attr)
                if not callable(value):
                    print(f"  {attr}: {value}")
            except:
                pass
except Exception as e:
    print(f"Error: {e}")
```

**`react-frontend/package.json`**
```json
{
  "name": "face-blur-frontend",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "axios": "^1.6.2",
    "react-scripts": "5.0.1"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  }
}
```

**`react-frontend/public/index.html`**
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#000000" />
    <meta name="description" content="Face Blur Application" />
    <title>Face Blur App</title>
  </head>
  <body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
  </body>
</html>
```

**`react-frontend/src/App.js`**
```javascript
import React, { useState } from "react";
import axios from "axios";
import "./App.css";

function App() {
  const [selectedFile, setSelectedFile] = useState(null);
  const [previewUrl, setPreviewUrl] = useState(null);
  const [resultUrl, setResultUrl] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [numFaces, setNumFaces] = useState(0);

  const handleFileSelect = (event) => {
    const file = event.target.files[0];
    if (file) {
      setSelectedFile(file);
      setResultUrl(null);
      setError(null);
      setNumFaces(0);

   
      const reader = new FileReader();
      reader.onload = (e) => {
        setPreviewUrl(e.target.result);
      };
      reader.readAsDataURL(file);
    }
  };

  const handleUpload = async () => {
    if (!selectedFile) {
      setError("Please select an image first");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const formData = new FormData();
      formData.append("image", selectedFile);

      const response = await axios.post(
        "http://localhost:3000/process",
        formData,
        {
          headers: {
            "Content-Type": "multipart/form-data",
          },
          timeout: 60000,
        }
      );

      const base64Image = response.data.image;
      const blob = base64ToBlob(base64Image, "image/jpeg");
      const url = URL.createObjectURL(blob);

      setResultUrl(url);
      setNumFaces(response.data.num_faces);
    } catch (err) {
      console.error("Error:", err);
      setError(
        err.response?.data?.error ||
          "Failed to process image. Make sure all services are running."
      );
    } finally {
      setLoading(false);
    }
  };

  const base64ToBlob = (base64, mimeType) => {
    const byteCharacters = atob(base64);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
      byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    return new Blob([byteArray], { type: mimeType });
  };

  const handleReset = () => {
    setSelectedFile(null);
    setPreviewUrl(null);
    setResultUrl(null);
    setError(null);
    setNumFaces(0);
  };

  return (
    <div className="App">
      <div className="container">
        <h1 className="title">Face Blur Application</h1>
        <p className="subtitle">
          Upload an image to automatically detect and blur faces
        </p>

        <div className="upload-section">
          <input
            type="file"
            accept="image/*"
            onChange={handleFileSelect}
            id="file-input"
            className="file-input"
          />
          <label htmlFor="file-input" className="file-label">
            Choose Image
          </label>

          {selectedFile && (
            <span className="file-name">{selectedFile.name}</span>
          )}
        </div>

        {previewUrl && (
          <div className="button-group">
            <button
              onClick={handleUpload}
              disabled={loading}
              className="info-badge-gradient"
            >
              {loading ? "Processing..." : "Process & Blur Faces"}
            </button>
            <button onClick={handleReset} className="info-badge-gradient">
              Reset
            </button>
          </div>
        )}

        {error && <div className="error-message">{error}</div>}

        <div className="images-container">
          {previewUrl && (
            <div className="image-box">
              <h3>Original Image</h3>
              <img src={previewUrl} alt="Original" />
            </div>
          )}

          {resultUrl && (
            <div className="image-box">
              <h3>Blurred Result</h3>
              <img src={resultUrl} alt="Blurred" />
              <div className="info-badge-gradient">
                {numFaces} wajah terdeteksi dan di-blur
              </div>
            </div>
          )}
        </div>

        {loading && (
          <div className="loading-overlay">
            <div className="spinner"></div>
            <p>Processing image with AI...</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
```

**`react-frontend/src/index.js`**
```javascript
import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

**`react-frontend/src/index.css`**
```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Oxygen",
    "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue",
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
}

code {
  font-family: source-code-pro, Menlo, Monaco, Consolas, "Courier New",
    monospace;
}
```

**`react-frontend/src/App.css`**
```css
.App {
  min-height: 100vh;
  padding: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.container {
  max-width: 1200px;
  width: 100%;
  background: white;
  border-radius: 20px;
  padding: 40px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.title {
  font-size: 2.5rem;
  color: #333;
  text-align: center;
  margin-bottom: 10px;
}

.subtitle {
  text-align: center;
  color: #666;
  font-size: 1.1rem;
  margin-bottom: 30px;
}

.upload-section {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 15px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.file-input {
  display: none;
}

.file-label {
  padding: 12px 30px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 10px;
  cursor: pointer;
  font-weight: 600;
  transition: transform 0.2s, box-shadow 0.2s;
  display: inline-block;
}

.file-label:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
}

.file-name {
  color: #333;
  font-weight: 500;
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.button-group {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-bottom: 30px;
  flex-wrap: wrap;
}

.btn {
  padding: 12px 30px;
  border: none;
  border-radius: 10px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-block;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(245, 87, 108, 0.4);
}

.btn-secondary {
  background: #e0e0e0;
  color: #333;
}

.btn-secondary:hover {
  background: #d0d0d0;
  transform: translateY(-2px);
}

.error-message {
  background: #fee;
  color: #c33;
  padding: 15px;
  border-radius: 10px;
  margin-bottom: 20px;
  text-align: center;
  font-weight: 500;
  border: 2px solid #fcc;
}

.images-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 30px;
  margin-top: 30px;
}

.image-box {
  background: #f8f9fa;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.image-box h3 {
  color: #333;
  margin-bottom: 15px;
  font-size: 1.3rem;
  text-align: center;
}

.image-box img {
  width: 100%;
  border-radius: 10px;
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.2);
}

.info-badge {
  margin-top: 15px;
  padding: 10px;
  background: linear-gradient(135deg, #84fab0 0%, #8fd3f4 100%);
  color: #006644;
  border-radius: 8px;
  text-align: center;
  font-weight: 600;
}

.info-badge-gradient {
  margin-top: 15px;
  padding: 12px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 8px;
  text-align: center;
  font-weight: 600;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
}

.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  color: white;
}

.spinner {
  width: 60px;
  height: 60px;
  border: 5px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-overlay p {
  font-size: 1.2rem;
  font-weight: 500;
}

@media (max-width: 768px) {
  .container {
    padding: 20px;
  }

  .title {
    font-size: 1.8rem;
  }

  .images-container {
    grid-template-columns: 1fr;
  }
}
```

Penjelasan:

- **Backend Rust**:
  - File utama: `rust-backend/src/main.rs` — mendefinisikan route `POST /process` (handler `process_image`) dan `GET /health`.
  - Alur `process_image`: menerima `multipart` upload (`image`), meneruskan data ke detektor Python (`http://localhost:5000/detect`) menggunakan `reqwest`, mendekode gambar base64 dari respons detektor, memanggil `process_image_with_blur` untuk menerapkan blur pada bounding box yang dikembalikan, lalu meng-encode kembali hasil ke base64 dan mengembalikan JSON `ProcessResponse`.
  - Pemrosesan gambar: `rust-backend/src/blur.rs` — fungsi `process_image_with_blur` membuat region wajah dari koordinat `FaceBox`, menerapkan Gaussian-like blur melalui `apply_gaussian_blur`, dan mengencode hasil ke JPEG.
  - Model data: `rust-backend/src/models.rs` berisi struct `FaceBox`, `DetectionResponse`, dan `ProcessResponse` (menggunakan `serde` untuk (de)serialisasi).

- **Detektor Python**:
  - File utama: `python-detector/app.py` — Flask app dengan endpoint `POST /detect` dan `GET /health`.
  - Alur `detect`: menerima file `image` di `request.files`, menggunakan OpenCV untuk decode, menjalankan model Ultralytics YOLO (`MODEL_PATH` menunjuk ke `../best (4).pt`), mengekstrak bounding box (`x1,y1,x2,y2`) dan confidence, lalu mengembalikan JSON yang berisi `faces` (array objek), `image` (base64 dari input/annotated image), `width`, `height`, dan `num_faces`.

- **Frontend React**:
  - File utama: `react-frontend/src/App.js` — UI untuk memilih file, menampilkan preview, dan mengirim `multipart/form-data` ke backend `http://localhost:3000/process`.
  - Respons: backend mengembalikan `image` (base64) dan `num_faces`; frontend mengonversi base64 menjadi Blob untuk ditampilkan sebagai gambar hasil.

Bagaimana prinsip pemrograman fungsional diterapkan di kode ini:
- **Fungsi kecil & fokus**: operasi utama dipisah menjadi fungsi terpisah — handler HTTP (`process_image`), deserialisasi/serialisasi (`models.rs`), deteksi (`python-detector/app.py`), dan pemrosesan gambar (`process_image_with_blur` di `blur.rs`).
- **Minimalkan state global**: tidak ada state aplikasi global yang dimodifikasi oleh pipeline; data (gambar, bounding box) dilewatkan sebagai argumen fungsi dan dikembalikan sebagai hasil.
- **Komposisi pipeline**: langkah-langkah disusun berurutan (decode upload → panggil detektor → proses region wajah → encode hasil) sehingga masing-masing langkah bisa diuji dan diganti secara terpisah.
- **Parallel iterators**: `rayon::prelude::*` digunakan di `blur.rs` (`par_iter`, `into_par_iter`) untuk mengeksekusi operasi pixel/region secara paralel tanpa menulis logika threading manual.

## Tangkapan Layar
![SS1](https://github.com/user-attachments/assets/396b4a7f-1687-40b3-b19a-39692a420d94)
*tampilan antarmuka unggah gambar.*

![SS2](https://github.com/user-attachments/assets/4a634544-38bc-4e44-a5c5-5b14b470f4f5)
*contoh hasil setelah area wajah diberi blur.*

---

## Kesimpulan
Proyek Face Blur Detection menggabungkan backend Rust untuk pemrosesan gambar yang cepat dan terprediksi dengan layanan Python (Ultralytics YOLO) untuk deteksi wajah, sehingga membentuk arsitektur terpisah yang mudah dioptimalkan dan diuji, pendekatan fungsional di Rust (fungsi kecil, imutabilitas lokal, komposisi) dan paralelisasi menggunakan `rayon` memberikan performa dan keterbacaan kode, sementara pemisahan ML ke layanan Python memudahkan eksperimen model dan mengurangi risiko pada jalur pemrosesan utama.

---

## Catatan Repositori
- Titik masuk layanan:
  - Backend Rust: `rust-backend/src/main.rs`
  - Pemrosesan gambar: `rust-backend/src/blur.rs`
  - Detektor Python: `python-detector/app.py`
  - Frontend React: `react-frontend/src/App.js`
- Berkas model yang ada di root repositori: `best (3).pt`, `best (4).pt`.
- Port yang digunakan secara default: detektor Python `5000`, backend Rust `3000`, frontend React `3001`.
