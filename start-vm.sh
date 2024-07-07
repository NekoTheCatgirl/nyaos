#!/bin/bash
docker build . -t nyaos
docker run --rm -v $PWD:/home/builder nyaos
