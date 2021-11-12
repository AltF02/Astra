FROM rust:latest

WORKDIR /usr/src/astra
COPY . .

RUN cargo install --path .

CMD ["astra"]
