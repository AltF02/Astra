FROM --platform=$BUILDPLATFORM rust:slim-bullseye

WORKDIR /usr/src/astra
COPY . .

RUN cargo install --path .

CMD ["astra"]
