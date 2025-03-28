build:
	@echo "Building the program..."
	cargo build --release --verbose

run:
	@echo "Running the program..."
	cargo run --release

clean:
	@echo "Cleaning up..."
	cargo clean

all: clean build run

debug:
	@echo "Building the program in debug mode..."
	cargo build --verbose