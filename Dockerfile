FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# ---------------------------------------------------------
# System dependencies
# ---------------------------------------------------------
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    cmake \
    git \
    libffi-dev \
    libxml2-dev \
    zlib1g-dev \
    libtinfo-dev \
    pkg-config

# ---------------------------------------------------------
# LLVM 17 for ARM64 (Apple Silicon Docker)
# ---------------------------------------------------------
RUN curl -LO https://github.com/llvm/llvm-project/releases/download/llvmorg-17.0.6/clang+llvm-17.0.6-aarch64-linux-gnu.tar.xz && \
    tar -xf clang+llvm-17.0.6-aarch64-linux-gnu.tar.xz && \
    mv clang+llvm-17.0.6-aarch64-linux-gnu /opt/llvm && \
    rm clang+llvm-17.0.6-aarch64-linux-gnu.tar.xz

ENV LLVM_HOME=/opt/llvm
ENV PATH="$LLVM_HOME/bin:${PATH}"
ENV LD_LIBRARY_PATH="$LLVM_HOME/lib:${LD_LIBRARY_PATH}"

# Required for clang-sys
ENV LLVM_SYS_170_PREFIX=/opt/llvm

# ---------------------------------------------------------
# Install Rust
# ---------------------------------------------------------
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# ---------------------------------------------------------
# Install c2rust
# ---------------------------------------------------------
RUN cargo install c2rust kani-verifier

WORKDIR /src
