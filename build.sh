#!/bin/sh
cargo build --target x86_64-pc-windows-msvc &&
    cp target/x86_64-pc-windows-msvc/debug/mygame.exe . &&
    exec ./mygame.exe "$@"
