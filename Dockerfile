FROM rust:1.56 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

CMD /usr/local/cargo/bin/alb-oidc-example
