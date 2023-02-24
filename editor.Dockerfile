FROM rust:bullseye

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install cargo-watch

RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash - &&\
apt-get install -y nodejs

RUN git clone https://github.com/fermyon/spin -b v0.9.0 && \
cd spin && \
rustup target add wasm32-wasi && \
cargo install --locked --path .

