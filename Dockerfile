FROM rust:1.60 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/rasa-action-server
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rasa-action-server /usr/local/bin/rasa-action-server

EXPOSE 8080

CMD ["rasa-action-server"]
