FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build
RUN cargo install cargo-watch

CMD sh -c "cargo watch -x check -x 'test -- --nocapture'"