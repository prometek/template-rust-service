# 1) Base image and system deps
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
      curl \
      build-essential \
      pkg-config \
      libssl-dev \
      libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# 2) Install Neovim (ARM64)
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim-linux-arm64.tar.gz && \
    tar -xzf nvim-linux-arm64.tar.gz -C /usr/local --strip-components=1 && \
    rm nvim-linux-arm64.tar.gz

# 3) Create a non-root 'vscode' user
ARG USER=vscode
RUN useradd -m ${USER}

# 4) Switch to that user and set up Rust
USER ${USER}
WORKDIR /home/${USER}/workspace

# 4a) Install rustup (minimal profile, stable channel)
RUN curl https://sh.rustup.rs -sSf | \
      sh -s -- -y --profile minimal --default-toolchain stable

# 4b) Put cargo/bin on the PATH so we can invoke rustup, cargo, rustc, etc.
ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

# 5) Add Clippy & Rustfmt
RUN rustup component add clippy rustfmt

# 6) Forward port and default command (optional)
EXPOSE 8000
