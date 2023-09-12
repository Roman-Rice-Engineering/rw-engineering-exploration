# DEV TARGET -- Designed to work with dev-server.sh script for running dev server

FROM rust:1.72-bullseye as dev

RUN USER=root mkdir rw-engineering

# Set up rust
RUN USER=root rustup target add wasm32-unknown-unknown
RUN USER=root cargo install trunk
RUN USER=root cargo install cargo-watch

# Install nginx -- Set up later
RUN USER=root apt-get update && apt-get install -y nginx


WORKDIR ./rw-engineering
COPY . .

# Set up nginx -- Installed earlier
RUN USER=root unlink /etc/nginx/sites-enabled/default
RUN USER=root cp ./docker/reverse-proxy.conf /etc/nginx/sites-available/reverse-proxy.conf
RUN USER=root ln -s /etc/nginx/sites-available/reverse-proxy.conf /etc/nginx/sites-enabled/reverse-proxy.conf


EXPOSE 80

CMD \
export BACKEND_URL="HELLO WORLD" && \
nginx && \
cd /rw-engineering/backend && cargo watch -x run & \
cd /rw-engineering/frontend && trunk serve & \ 
bash
