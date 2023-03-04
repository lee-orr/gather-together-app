FROM ubuntu:rolling

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    wget

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install cargo-watch
RUN rustup component add clippy
RUN rustup component add rustfmt

RUN <<EOF
    cargo install nu --features=dataframe
    mkdir -R /root/.config/nushell
    cd /root/.config/nushell
    wget https://github.com/nushell/nushell/blob/main/crates/nu-utils/src/sample_config/default_env.nu
    wget https://github.com/nushell/nushell/blob/main/crates/nu-utils/src/sample_config/default_config.nu
    mv ./default_env.nu ./env.nu
    mv ./default_config.nu ./config.nu
EOF

RUN cargo install --locked zellij
RUN cargo install mprocs
RUN cargo install --locked bacon
RUN cargo install gitui


RUN 
RUN <<EOF
cargo install rtx-cli
rtx install nodejs@19
echo 'eval "$(rtx activate bash)"' >> ~/.bashrc
EOF

RUN cargo install wash-cli
RUN bash -c "$(curl -fsSL https://cosmonic.sh/install.sh)"
ENV PATH="/root/.cosmo/bin:${PATH}"

RUN apt-get install -y git

RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi

ENV CARGO_TARGET_DIR="./target"