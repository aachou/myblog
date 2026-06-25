FROM rust:slim-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/myblog .
COPY templates ./templates
COPY static ./static
COPY pages ./pages
COPY posts ./posts
COPY config ./config
EXPOSE 3000
CMD ["./myblog"]
