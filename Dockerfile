FROM docker.io/rust:1.67

WORKDIR /usr/src/ole
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --path backend

CMD ["./wrapper.sh"]
