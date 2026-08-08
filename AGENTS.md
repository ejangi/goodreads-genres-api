# Project guidance

- Use the Docker toolchain for Rust commands: the host environment may not have
  `cargo` installed. Run the test suite with `docker compose run --rm app cargo test`.
- `Dockerfile` produces the production image; `docker/dev.Dockerfile` and
  `compose.yml` support local development.
- Tests fetch public Goodreads pages and a Google Cloud Storage HTML fixture.
  They require network access and intentionally verify the scraper against live
  external content, which can change independently of this repository.
- Keep `Cargo.lock` committed whenever dependency versions change.
