#!/bin/bash
docker build . -t nyaos
docker run --rm -it -v $PWD:/root/env nyaos
