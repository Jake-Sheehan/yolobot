default:
    cargo build
    cargo run -p yolobot_core

test:
    cargo test

watch:
    cargo watch -x run 
