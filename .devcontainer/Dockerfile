FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

##### Backend dependencies #####
RUN apt-get update \
    && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libssl3 \
    postgresql-client \
    openssh-server

RUN service ssh start
RUN echo 'vscode:vscode' | chpasswd \
    && sed -i 's/^#*PasswordAuthentication.*/PasswordAuthentication yes/' /etc/ssh/sshd_config
EXPOSE 22
RUN /usr/sbin/sshd

# Install sea-orm-cli
RUN cargo install diesel_cli
