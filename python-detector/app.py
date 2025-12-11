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
                # Get coordinates
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
