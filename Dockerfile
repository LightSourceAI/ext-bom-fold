# Packages the BOM fold tool for cross-platform usage.
FROM rust:1.68-alpine AS builder_base

RUN USER=root apk add --no-cache \
  ca-certificates \
  perl \
  make \
  protobuf-dev \
  lld \
  clang-dev \
  musl-dev

COPY . .
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin fold_items

FROM builder_base AS cli
ARG APP_USER=ls-user
RUN addgroup -S "${APP_USER}" && adduser -S "${APP_USER}" -G "${APP_USER}"

COPY --from=builder_base /target/release/fold_items /fold_items

USER ${APP_USER}
ENTRYPOINT [ "/fold_items" ]
