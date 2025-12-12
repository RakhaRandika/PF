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

      // Create preview
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
          timeout: 60000, // 60 seconds timeout
        }
      );

      // Convert base64 to blob URL
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
        <h1 className="title">Face Blur detection</h1>
        <p className="subtitle">
          Upload foto untuk mendeteksi dan mengaburkan wajah secara otomatis
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
            Pilih Gambar
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
              {loading ? "Processing..." : "proses Gambar"}
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
            <p>loading ...</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
