@echo off
cd /d "%~dp0..\.."
cargo run --locked --bin project-rl -- --apercu-finition-visuelle
if errorlevel 1 pause
