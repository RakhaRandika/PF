@echo off
echo Starting Face Blur Application Services...
echo.

echo Starting Python Detector (Port 5000)...
start "Python Detector" cmd /k "cd python-detector && python app.py"

timeout /t 3 /nobreak >nul

echo Starting Rust Backend (Port 3000)...
start "Rust Backend" cmd /k "cd rust-backend && cargo run --release"

timeout /t 3 /nobreak >nul

echo Starting React Frontend (Port 3001)...
start "React Frontend" cmd /k "cd react-frontend && npm start"

echo.
echo All services are starting...
echo - Python Detector: http://localhost:5000
echo - Rust Backend: http://localhost:3000
echo - React Frontend: http://localhost:3001
echo.
echo Press any key to exit this window (services will continue running)...
pause >nul
