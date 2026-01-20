FROM rust:latest
RUN rustup conponent add rust-analyzer rustfmt clippy
WORKDIR /home/workshop
COPY . .
RUN apt update && apt install -y git curl
CMD ["sleep", "infinity"]