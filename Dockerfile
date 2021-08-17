FROM rust:1.54-buster

WORKDIR /usr/src/astra
COPY . .

RUN cargo install --path .

CMD ["astra"]
