build:
	@echo "Building the program..."
	cargo build --release --verbose --bin fec

run:
	@echo "Running the program..."
	cargo run --release

clean:
	@echo "Cleaning up..."
	cargo clean

all: clean build run