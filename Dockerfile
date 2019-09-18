FROM rust:1.37 as build
RUN USER=root cargo new --bin health-checker
WORKDIR /health-checker

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/health-checker
RUN cargo build --release

FROM scratch
COPY --from=build /health-checker/target/release/health-checker .
CMD ["ls"]
ENTRYPOINT ["health-checker"]