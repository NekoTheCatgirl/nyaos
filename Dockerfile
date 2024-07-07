# Use the latest Arch Linux base image
FROM archlinux:latest

# Install necessary dependencies
RUN pacman -Syu xorriso rustup nasm make grub clang git base-devel sudo --noconfirm

# Create a non-root user to build AUR packages
RUN useradd -m builder && echo "builder ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder

# Switch to the builder user
USER builder

# Setup volumes
VOLUME  /home/builder
WORKDIR /home/builder

# Update rust
RUN rustup default nightly

# Add the required target
RUN rustup target add x86_64-unknown-none

# Import the GPG key for binutils
RUN gpg --keyserver keyserver.ubuntu.com --recv-keys 13FCEF89DD9E3C4F

# Clone the AUR x86_64-elf-binutils
RUN git clone https://aur.archlinux.org/x86_64-elf-binutils.git

# Build and install x86_64-elf-binutils
RUN cd x86_64-elf-binutils && makepkg -si --noconfirm && cd .. && rm -rf x86_64-elf-binutils

# Import the GPG key for gcc
RUN gpg --keyserver keyserver.ubuntu.com --recv-keys 6C35B99309B5FA62

# Clone the AUR x86_64-elf-gcc
RUN git clone https://aur.archlinux.org/x86_64-elf-gcc.git

# Build and install x86_64-elf-gcc
RUN cd x86_64-elf-gcc && makepkg -si --noconfirm && cd .. && rm -rf x86_64-elf-gcc

# Switch back to root user
USER root

# Add your script
COPY make.sh /usr/local/bin/make.sh
RUN chmod +x /usr/local/bin/make.sh

# Verify installation
RUN x86_64-elf-ld --version

# Dirty hack because i dont want to recompile it all right now
RUN pacman -Sy mtools --noconfirm

# Run the shell script when the container starts
ENTRYPOINT ["/usr/local/bin/make.sh"]