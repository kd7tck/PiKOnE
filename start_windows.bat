@echo off
setlocal

echo ==========================================
echo PiKOnE Windows Auto-Setup and Run Script
echo ==========================================
echo.

REM 1. Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed.
    echo Please install Rust first by downloading and running rustup-init.exe from:
    echo https://rustup.rs/
    echo.
    echo When installing, you can select the default options.
    echo.
    pause
    exit /b 1
)

REM 2. Install the GNU toolchain
REM This avoids the need for Visual Studio C++ Build Tools
echo [INFO] Checking for GNU toolchain (this avoids VS Build Tools)...
rustup toolchain install stable-x86_64-pc-windows-gnu

echo.
echo [INFO] Building and starting PiKOnE...
echo [INFO] This might take a few minutes the first time.
echo.

REM 3. Run the application using the GNU toolchain explicitly
REM Using +stable-x86_64-pc-windows-gnu ensures the bundled linker is found
cargo +stable-x86_64-pc-windows-gnu run

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Something went wrong during the build.
    echo Please check the error messages above.
    pause
    exit /b 1
)

pause
