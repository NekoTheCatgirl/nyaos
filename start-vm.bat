docker build . -t nyaos
docker run --rm -v %cd%:/home/builder nyaos
