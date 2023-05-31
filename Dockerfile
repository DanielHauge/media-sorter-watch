FROM rust:latest as builder
COPY . .
RUN cargo install --path .

ENTRYPOINT [ "media-sorter-watch" ]