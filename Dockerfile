FROM rust:1.100-bookworm AS build

WORKDIR /src

COPY . .
RUN cargo build --release --locked

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=build /src/target/release/mitsuami-unit-rust /unit

EXPOSE 8080
ENTRYPOINT ["/unit"]
