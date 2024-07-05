#!/bin/bash

# This shell script assumes you have make, nasm, xorriso, grub-mkrescue and rust installed. If any are missing it will likely error out!
# Tip: Use the ./start-vm.sh or ./start-vm.bat file to enter a build environment that will function as expected
make prepare
make build
make link
make finish
make cleanup