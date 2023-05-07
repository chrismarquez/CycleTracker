# syntax=docker/dockerfile:1.3-labs
FROM rust:1.69 as build
LABEL authors="christopher & pug"
ARG CRATE_NAME=CycleTracker

# capture and compile dependencies on a blank project
WORKDIR ./app/${CRATE_NAME}
RUN cargo init .
COPY Cargo.lock Cargo.toml Rocket.toml ./
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# now compile only the source code of our project
COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
    set -e
    touch /app/${CRATE_NAME}/src/main.rs
    cargo build --release
EOF


FROM debian:11-slim
ARG CRATE_NAME=CycleTracker
ENV ROCKET_PROFILE=release
COPY --from=build app/${CRATE_NAME}/target/release/${CRATE_NAME} .
COPY --from=build app/${CRATE_NAME}/Rocket.toml .
CMD ["./CycleTracker"]

# https://gist.github.com/noelbundick/6922d26667616e2ba5c3aff59f0824cd --> Caching for layers

# Prior caching:
#real    1m53.365s
#user    0m0.469s
#sys     0m0.281s

# V2: caching but not modifying layers nor order, therefore changing any file will trigger a new build :(
#real    0m1.420s/1m32.781s
#user    0m0.320s
#sys     0m0.232s

# V3 multistage build, now modifications only affect either dependencies or our project
# we take about 1m17s to compile dependencies in case of Cargo.lock/toml, Dockerfile or Rocket.toml modifications
# or about 16s (depends on size of project) to recompile CycleTracker source

