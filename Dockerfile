FROM rust AS rust

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=rust /app/target/release/app ./

ENV PORT 8080
ENV ROCKET_PORT ${PORT}
ENV ROCKET_ADDRESS 0.0.0.0

CMD ["./app"]