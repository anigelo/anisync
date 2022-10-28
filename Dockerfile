FROM rust:1.64.0-alpine3.16 as builder
WORKDIR /build
COPY ./Cargo.toml ./Cargo.toml
RUN cargo fetch
COPY . .
RUN apk add --no-cache build-base nodejs npm
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
COPY --from=builder /build/target/release/anisync ${APP}/anisync
COPY --from=builder /build/www/dist ${APP}/www
WORKDIR ${APP}
CMD ["./anisync"]
