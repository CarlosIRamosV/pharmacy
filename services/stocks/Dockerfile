FROM rust:alpine as builder

WORKDIR /usr/src/app

COPY . .

# Install musl-dev to build the binary
RUN apk add musl-dev
RUN cargo build --bin stocks --release

FROM alpine:latest

RUN apk add --no-cache curl openssl

COPY --from=builder /usr/src/app/target/release/stocks /usr/local/bin/stocks

EXPOSE 8080

CMD ["stocks"]