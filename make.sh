#!/bin/bash

echo "Running the build system! This may take a while, depending on your system."
echo "Starting in 3..."
sleep 1
echo "2..."
sleep 1
echo "1..."
sleep 1
echo "Lets build it!"
# This shell script assumes you have make, nasm, xorriso, grub-mkrescue and rust installed. If any are missing it will likely error out!
# Tip: Use the ./start-vm.sh or ./start-vm.bat file to enter a build environment that will function as expected
make prepare
make build
make link
make finish
make cleanup

echo "The build process should be complete (check output for errors)"