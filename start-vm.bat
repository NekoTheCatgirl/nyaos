docker build . -t nyaos
docker run --rm -it -v %cd%:/root/env nyaos
