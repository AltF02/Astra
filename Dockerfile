FROM --platform=$BUILDPLATFORM rust:1.57-slim-buster

WORKDIR /usr/src/astra
COPY . .

RUN cargo install --path .

CMD ["astra"]
