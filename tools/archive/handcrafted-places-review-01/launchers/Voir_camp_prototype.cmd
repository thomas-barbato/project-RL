@echo off
setlocal
cd /d "%~dp0"
if not exist "target\debug\examples\handcrafted_camp_full_review.exe" (
    cargo build --locked --example handcrafted_camp_full_review
    if errorlevel 1 (
        pause
        exit /b 1
    )
)
"target\debug\examples\handcrafted_camp_full_review.exe"
