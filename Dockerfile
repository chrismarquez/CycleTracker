# syntax=docker/dockerfile:1.3-labs
FROM rust:1.69 as build
LABEL authors="christopher & pug"
ARG TARGET_PLATFORM=aarch64-apple-darwin
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && \
    mv ./target/release/CycleTracker .

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
    set -e
    touch /src/main.rs
    cargo build --release
EOF


FROM debian:11-slim
ENV ROCKET_PROFILE=staging
COPY --from=build target/release/CycleTracker .
COPY --from=build Rocket.toml .
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
