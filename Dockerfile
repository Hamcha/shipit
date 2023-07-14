FROM public.ecr.aws/docker/library/rust:1-alpine as builder

RUN rustup default stable
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf git

# Set `SYSROOT` to a dummy path (default is /usr) because pkg-config-rs *always*
# links those located in that path dynamically but we want static linking, c.f.
# https://github.com/rust-lang/pkg-config-rs/blob/54325785816695df031cef3b26b6a9a203bbc01b/src/lib.rs#L613
ENV SYSROOT=/dummy

WORKDIR /wd
COPY . /wd
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM public.ecr.aws/docker/library/alpine
ARG version=unknown
ARG release=unreleased

COPY --from=builder /wd/target/x86_64-unknown-linux-musl/release/shipit /usr/local/bin/
CMD ["shipit"]
