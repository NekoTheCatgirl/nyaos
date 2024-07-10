# Use the randomdude gcc crosscompiler base image
FROM randomdude/gcc-cross-x86_64-elf

# Update package repositories and install necessary dependencies
RUN apt-get update 
RUN apt-get upgrade -y
RUN apt-get install -y nasm
RUN apt-get install -y xorriso
RUN apt-get install -y grub-pc-bin
RUN apt-get install -y grub-common
RUN apt-get install -y curl pkg-config

# Setup volumes
VOLUME  /root/env
WORKDIR /root/env

# Install rustup and Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust binaries to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Update rust
RUN rustup default nightly

# Add the required target
RUN rustup target add x86_64-unknown-none

# Add your script
COPY ./build-tools/make.sh /usr/local/bin/make.sh
RUN chmod +x /usr/local/bin/make.sh

# Run the shell script when the container starts
ENTRYPOINT ["/usr/local/bin/make.sh"]