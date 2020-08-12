FROM rust:1.45.0-stretch as build

ARG SCCACHE_ENDPOINT
ARG SCCACHE_BUCKET
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID

RUN apt-get update &&\
  apt-get install -y cmake --no-install-recommends &&\
  apt-get clean && rm -rf /var/lib/apt/lists/*


WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
RUN mkdir src &&\
  echo "pub fn main() {}" > src/main.rs &&\
  cargo build --release 

RUN rm -rf /build/src
RUN rm -rf /build/target/release/.fingerprint/css-kafka-parsing-service-*

# Now copy over the source and compile our app
COPY src/ /build/src/
RUN cargo build --release

# ==================================================

FROM debian:stretch-20200607 as service

# RUN apt-get update &&\
#   apt-get install -y libreoffice --no-install-recommends &&\
#   apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY log4rs.yml /app/
COPY --from=build /build/target/release/css-kafka-parsing-service /app

CMD ["./rust-graphql-example"]
