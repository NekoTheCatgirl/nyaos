.PHONY: prepare build link finish cleanup

x86_64_asm_source_files := $(shell find asm-src -name *.asm)
x86_64_asm_object_files := $(patsubst asm-src/%.asm, build-artifacts/%.o, $(x86_64_asm_source_files))

$(x86_64_asm_object_files): build-artifacts/%.o : asm-src/%.asm
	nasm -f elf64 $(patsubst build-artifacts/%.o, asm-src/%.asm, $@) -o $@

x86_64_rust_object_files := $(shell find target/x86_64-nyaos-none/debug/ -name libnyaos.a)

prepare:
	@echo "Preparing directories"
	rm -rf ./build
	mkdir ./build
	mkdir ./build-artifacts

build: $(x86_64_asm_object_files)
	@echo "Building"
	cargo rustc --target=x86_64-nyaos-none.json -- --emit=obj

link:
	@echo "Linking..."
	ld -n --gc-sections -o build-artifacts/kernel.bin -T targets/x86_64/linker.ld $(x86_64_asm_object_files) $(x86_64_rust_object_files)

finish:
	@echo "Finishing up, moving files into the right areas!"
	cp build-artifacts/kernel.bin targets/x86_64/iso/boot/kernel.bin
	grub-mkrescue /usr/lib/grub/i386-pc -o build/kernel.iso targets/x86_64/iso

cleanup:
	@echo "Cleaning up build artifacts"
	cargo clean
	rm -rf ./build-artifacts
	rm -rf ./targets/x86_64/iso/boot/kernel.bin