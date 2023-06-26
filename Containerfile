FROM docker.io/rust:1.67

WORKDIR /usr/src/ole
COPY . .

RUN cargo install trunk
RUN cargo install --path backend

CMD ["./wrapper.sh"]
