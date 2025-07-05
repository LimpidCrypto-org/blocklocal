FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

##### Backend dependencies #####
RUN apt-get update \
    && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libssl3 \
    postgresql-client

# Install sea-orm-cli
RUN cargo install diesel_cli
