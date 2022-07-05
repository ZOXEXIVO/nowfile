FROM rust:1.62 as build
WORKDIR /src

COPY ./ ./

RUN cargo test

RUN cargo build --release

FROM rust:1.62-slim
WORKDIR /app
COPY --from=build /src/target/release .

ENV ENDPOINT http://172.17.0.1:4572

ENTRYPOINT ["./nowfile"]