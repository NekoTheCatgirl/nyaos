FIN_DIR = /build

.PHONY: build link finish cleanup

build:
	@echo "Building"

link:
	@echo "Linking..."

finish:
	@echo "Finishing up, moving files into the right areas!"
	touch ./build/test.txt

cleanup:
	@echo "Cleaning up build artifacts"