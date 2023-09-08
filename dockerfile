FROM rust:latest

RUN apt update \
    && apt install -y telnet netcat-openbsd
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1