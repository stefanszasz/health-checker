FROM rust:1.38 as build
# create a new empty shell project
RUN USER=root cargo new --bin health-checker
WORKDIR /health-checker

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/health-checker*
RUN cargo build --release

# our final base
FROM scrath

# copy the build artifact from the build stage
COPY --from=build /health-checkert/target/release/health-checker .

CMD ["./health-checker", "-v"]
# set the startup command to run your binary

#FROM alpine:3.9.2
#
#ADD target/debug/health-checker .
#
#CMD ["./health-checker", "-v"]