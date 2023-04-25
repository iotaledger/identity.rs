FROM alpine

RUN apk --no-cache add nodejs npm curl pkgconfig openssl-dev build-base
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source "$HOME/.cargo/env"
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli

