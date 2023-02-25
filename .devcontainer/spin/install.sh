git clone https://github.com/fermyon/spin -b v0.8.0
cd spin
cargo install --locked --path .
spin templates install --git https://github.com/fermyon/spin --update