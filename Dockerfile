FROM ubuntu:rolling

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

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
RUN echo "export PATH=\"/root/.cosmo/bin:\${PATH}\"" >> "${HOME}/.bashrc" && source "${HOME}/.bashrc"