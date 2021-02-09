FROM rust

# using dummy.rs to avoid recompiling dependencies at every build
# https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build
COPY docker-dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#docker-dummy.rs#' Cargo.toml
RUN cargo build --release --bin server
RUN sed -i 's#docker-dummy.rs#src/main.rs#' Cargo.toml

# actually compiling changed code
COPY . .
RUN cargo build --release --bin server

EXPOSE 9001:9001
ENTRYPOINT ["./target/release/server"]
