ARG REDLIB_IMAGE_PLATFORM=linux/amd64

FROM --platform=$REDLIB_IMAGE_PLATFORM ghcr.io/rust-cross/rust-musl-cross:x86_64-musl@sha256:6c3c52df33dbd3fa999455c56db5be6fe2a9df5af63e00388194d936fd5cd003 AS builder

WORKDIR /src

RUN apt-get update \
	&& apt-get install -y --no-install-recommends git cmake perl pkg-config libclang-dev \
	&& rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY static ./static
COPY templates ./templates

RUN cargo build --release --locked --bin redlib --target x86_64-unknown-linux-musl

FROM --platform=$REDLIB_IMAGE_PLATFORM alpine:3.23@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11 AS runtime

LABEL org.opencontainers.image.title="Redlib" \
	  org.opencontainers.image.description="Proofline-operated Redlib fork for a best-effort public community service" \
	  org.opencontainers.image.source="https://github.com/open-proofline/redlib" \
	  org.opencontainers.image.licenses="AGPL-3.0-only"

RUN apk add --no-cache ca-certificates wget \
	&& addgroup -S redlib \
	&& adduser -S -D -H -h /nonexistent -s /sbin/nologin -G redlib redlib

COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/redlib /usr/local/bin/redlib

USER redlib
EXPOSE 8080

HEALTHCHECK --interval=1m --timeout=3s --start-period=10s --retries=3 \
	CMD wget --spider -q http://localhost:8080/settings || exit 1

CMD ["redlib"]
