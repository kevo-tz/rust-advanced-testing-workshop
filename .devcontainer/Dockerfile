FROM kevotz/rust_python_node:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add rustfmt clippy \
    && cargo install --locked workshop-runner \
    # && cargo install --path ctr \
    && apt install -y zsh

CMD [ "zsh" ]
