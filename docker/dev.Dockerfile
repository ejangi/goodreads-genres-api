FROM rust:latest

WORKDIR /app

ENV RUST_BACKTRACE 1

COPY . .

RUN cargo build
RUN cargo install cargo-watch

CMD sh -c "cargo watch -x 'test -- --nocapture'"