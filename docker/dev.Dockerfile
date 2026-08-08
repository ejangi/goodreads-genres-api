FROM rust:1.97.1-bookworm

WORKDIR /app

ENV RUST_BACKTRACE=1
ENV PORT=8080
ENV ROCKET_ADDRESS=0.0.0.0

COPY . .

RUN cargo build
RUN cargo install cargo-watch --version 8.5.3 --locked

CMD sh -c "cargo watch -x 'test -- --nocapture'"
