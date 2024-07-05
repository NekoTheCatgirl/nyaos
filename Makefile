FIN_DIR = /build

.PHONY: prepare build link finish cleanup

prepare:
	@echo "Preparing directories"
	@rm -rf ./build
	@mkdir ./build
	@mkdir ./build-artifacts

build:
	@echo "Building"
	@cargo rustc --target=x86_64-unknown-none -- --emit=obj

link:
	@echo "Linking..."

finish:
	@echo "Finishing up, moving files into the right areas!"
	

cleanup:
	@echo "Cleaning up build artifacts"
	@cargo clean
	@rm -rf ./build-artifacts