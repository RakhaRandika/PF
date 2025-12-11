# Face Blur Application

Aplikasi deteksi dan blur wajah dengan arsitektur multi-layer:

- **Python (Flask)**: Deteksi wajah menggunakan YOLO model
- **Rust (Axum)**: Blur processing dengan Rayon multi-processing
- **React**: Frontend UI untuk upload dan display hasil

## Arsitektur Sistem

```
[React Frontend] → [Rust Backend] → [Python Detector]
     ↓                    ↓                 ↓
  Upload foto      Process blur      Detect faces
  Display hasil    (Rayon parallel)  (YOLO model)
```

## Alur Kerja

1. User upload foto di React frontend
2. Frontend kirim foto ke Rust backend (port 3000)
3. Rust backend forward foto ke Python detector (port 5000)
4. Python detector menggunakan YOLO model untuk detect wajah
5. Python return koordinat wajah + base64 image
6. Rust backend process blur pada area wajah menggunakan Rayon (parallel processing)
7. Rust return hasil blur ke frontend
8. Frontend display hasil blur

## Setup dan Installation

### 1. Python Detector Service

```powershell
cd python-detector

# Install dependencies
pip install -r requirements.txt

# Run service
python app.py
```

Service akan running di `http://localhost:5000`

### 2. Rust Backend

```powershell
cd rust-backend

# Build dan run
cargo run --release
```

Backend akan running di `http://localhost:3000`

### 3. React Frontend

```powershell
cd react-frontend

# Install dependencies
npm install

# Run development server
npm start
```

Frontend akan running di `http://localhost:3001` (atau port yang tersedia)

## Menjalankan Aplikasi

1. **Start Python Detector** (Terminal 1):

   ```powershell
   cd python-detector
   python app.py
   ```

2. **Start Rust Backend** (Terminal 2):

   ```powershell
   cd rust-backend
   cargo run --release
   ```

3. **Start React Frontend** (Terminal 3):

   ```powershell
   cd react-frontend
   npm start
   ```

4. Buka browser ke `http://localhost:3001`
5. Upload foto yang mengandung wajah
6. Klik "Process & Blur Faces"
7. Hasil blur akan ditampilkan

## Fitur

✅ Deteksi wajah otomatis menggunakan YOLO
✅ Blur processing parallel dengan Rayon
✅ UI yang user-friendly
✅ Preview original dan hasil blur side-by-side
✅ Informasi jumlah wajah yang terdeteksi
✅ Support berbagai format gambar

## Teknologi

- **Python**: Flask, OpenCV, Ultralytics YOLO, NumPy
- **Rust**: Axum, Rayon, Image, Tokio
- **React**: Axios, React Hooks

## Model

Aplikasi menggunakan model YOLO custom (`best (3).pt`) untuk deteksi wajah.
Pastikan model file berada di root directory workspace.

## Port Configuration

- Python Detector: 5000
- Rust Backend: 3000
- React Frontend: 3001 (default React port)

## Notes

- Pastikan semua 3 services running sebelum menggunakan aplikasi
- Rust backend menggunakan Rayon untuk parallel processing blur regions
- Python detector menggunakan confidence threshold 0.5 untuk deteksi wajah
