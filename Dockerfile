FROM arm64v8/rust as build
RUN rustup target add aarch64-unknown-linux-gnu

# create a new empty shell project
RUN USER=root cargo new --bin rustdemo
WORKDIR /rustdemo

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rustdemo*
RUN cargo build --release --target aarch64-unknown-linux-gnu

# our final base
FROM arm64v8/debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /rustdemo/target/aarch64-unknown-linux-gnu/release/rustdemo .

# set the startup command to run your binary
CMD ["./rustdemo"]