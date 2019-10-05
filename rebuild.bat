@echo off
cls
cmd /c cargo clean
cmd /c rustup run nightly-i686-pc-windows-msvc cargo build