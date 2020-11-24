WASM=target/wasm32-unknown-unknown/release/demo.wasm
DEST_WASM=demo.wasm

# Build the WebAssembly file. This isn't the interesting part yet.
.PHONY: wasm
wasm:
	cargo build --release --target wasm32-unknown-unknown

# Now, copy the build .wasm file to the webpage directory.
# Using make is necessary because cargo does not yet provide post-build copy
.PHONY: install
install: wasm
	cp $(WASM) $(DEST_WASM)

# webserver is a shortcut for:
# 1. Change to wasm-demo directory
# 2. Run local webserver ruby script to host wasm demo on localhost:8000
.PHONY: webserver
webserver: install
	ruby webserver.rb

# The shared memory IPC buffers that are required to move strings across
# WebAssembly virtual machine boundary ARE NOT THREAD SAFE. Using normal
# multi-threaded testing will fail horribly. Must use only one thread.
.PHONY: test
test:
	cargo test -- --test-threads 1

.PHONY: clean
clean:
	cargo clean
