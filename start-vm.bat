docker build . -t nyaos
docker run --rm -v %cd%:/root/env nyaos
