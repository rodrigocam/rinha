FROM rust:1.72

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/rinha"]
