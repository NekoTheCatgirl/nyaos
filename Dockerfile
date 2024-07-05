FROM archlinux:latest

RUN pacman -Syu xorriso rustup nasm make --noconfirm

RUN rustup default nightly

RUN rustup target add x86_64-unknown-none

VOLUME  /root/env
WORKDIR /root/env
