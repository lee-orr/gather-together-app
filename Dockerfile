FROM rust:bullseye as Base

RUN <<EOF
cargo install nu --features=dataframe
cargo install --locked zellij
cargo install mprocs
cargo install --locked bacon
cargo install gitui
cargo install rtx-cli
EOF

RUN cargo install cargo-watch

RUN rustup component add clippy
RUN rustup component add rustfmt

RUN <<EOF
rtx install nodejs@19
echo 'eval "$(~/bin/rtx activate bash)"' >> ~/.bashrc
EOF

RUN cargo install wash-cli