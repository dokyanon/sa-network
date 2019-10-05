@echo off
cls
cmd /c rustup run nightly-i686-pc-windows-msvc cargo build
pause
