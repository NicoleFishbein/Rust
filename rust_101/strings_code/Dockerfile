# Use a Linux base image with Rust installed
FROM rust:latest

# Install dependencies needed for cross-compiling and other tools
RUN apt update && apt upgrade -y && \
    apt install -y clang lld make

# Install the Rust target for Windows ARM64
RUN rustup target add aarch64-pc-windows-msvc

# Set the linker to lld-link (from LLVM), which can handle Windows binaries
ENV CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_LINKER=lld-link

# Set up the working directory
WORKDIR /app

# Default command to compile the project for Windows ARM64
CMD ["cargo", "build", "--target", "aarch64-pc-windows-msvc"]

