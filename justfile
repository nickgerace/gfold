test:
    cd lib/libgfold && cargo test --release -- --nocapture
    cd lib/libgfold-v5 && cargo test --release -- --nocapture
