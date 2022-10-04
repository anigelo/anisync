FROM rust:1.64 as builder
# Cache deps
RUN USER=root cargo new --bin anisync
WORKDIR ./anisync
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Build code
ADD . ./
RUN rm ./target/release/deps/anisync*
RUN cargo build --release


FROM debian:bookworm-slim
ARG APP=/usr/src/app
VOLUME /media
VOLUME /data
EXPOSE 8025
# Install megacmd
RUN apt-get update \
    && apt-get -y install curl gnupg2 ca-certificates tzdata \
    && update-ca-certificates \
    && curl https://mega.nz/linux/repo/Debian_testing/amd64/megacmd-Debian_testing_amd64.deb --output /tmp/megacmd.deb \
    && apt install /tmp/megacmd.deb -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/megacmd.*
# Copy binaries from builder image
COPY --from=builder /anisync/target/release/anisync ${APP}/anisync
WORKDIR ${APP}
CMD ["./anisync"]
