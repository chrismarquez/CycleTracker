FROM rust:1.69 as build
LABEL authors="christopher & pug"
COPY . .
RUN cargo build --release

FROM debian:11-slim
ENV ROCKET_PROFILE=staging
COPY --from=build target/release/CycleTracker .
COPY --from=build Rocket.toml .
CMD ["./CycleTracker"]


# https://gist.github.com/noelbundick/6922d26667616e2ba5c3aff59f0824cd --> Caching for layers