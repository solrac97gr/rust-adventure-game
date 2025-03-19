# Configuration variables
PROJECT_NAME = adventure-game
CARGO = cargo
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug
CSV_FILE = history.csv

# Default target
.PHONY: all
all: build

# Build the project in debug mode
.PHONY: build
build:
	$(CARGO) build

# Build the project in release mode
.PHONY: release
release:
	$(CARGO) build --release

# Run the project in debug mode
.PHONY: run
run: build
	$(CARGO) run

# Run the project in release mode
.PHONY: run-release
run-release: release
	./$(RELEASE_DIR)/$(PROJECT_NAME)

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Check code for errors without building
.PHONY: check
check:
	$(CARGO) check

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Format code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Check if CSV file exists, create a simple one if not
.PHONY: check-csv
check-csv:
	@if [ ! -f $(CSV_FILE) ]; then \
		echo "Creating sample $(CSV_FILE)..."; \
		echo "SITUACION;INICIO;You are at a crossroads. Which path will you take?;0" > $(CSV_FILE); \
		echo "OPCION;FOREST;Go into the dark forest;0" >> $(CSV_FILE); \
		echo "OPCION;MOUNTAIN;Climb the mountain;0" >> $(CSV_FILE); \
		echo "SITUACION;FOREST;You enter the forest and find a mysterious potion.;-10" >> $(CSV_FILE); \
		echo "OPCION;DRINK;Drink the potion;0" >> $(CSV_FILE); \
		echo "OPCION;LEAVE;Leave it and continue;0" >> $(CSV_FILE); \
		echo "SITUACION;MOUNTAIN;You climb the mountain and find a treasure chest.;-5" >> $(CSV_FILE); \
		echo "OPCION;OPEN;Open the chest;0" >> $(CSV_FILE); \
		echo "OPCION;IGNORE;Ignore it and continue;0" >> $(CSV_FILE); \
		echo "SITUACION;DRINK;You drink the potion and feel stronger!;20" >> $(CSV_FILE); \
		echo "SITUACION;LEAVE;You continue your journey through the forest.;0" >> $(CSV_FILE); \
		echo "SITUACION;OPEN;The chest contains gold coins!;10" >> $(CSV_FILE); \
		echo "SITUACION;IGNORE;You ignore the chest and continue your journey.;0" >> $(CSV_FILE); \
	fi

# Prepare for distribution
.PHONY: dist
dist: release check-csv
	mkdir -p dist
	cp $(RELEASE_DIR)/$(PROJECT_NAME) dist/
	cp $(CSV_FILE) dist/
	cp README.md dist/
	@echo "Distribution files created in 'dist' directory"

# Install the game (requires sudo)
.PHONY: install
install: release
	sudo cp $(RELEASE_DIR)/$(PROJECT_NAME) /usr/local/bin/

# Help command
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all          - Default target, same as 'build'"
	@echo "  build        - Build the project in debug mode"
	@echo "  release      - Build the project in release mode"
	@echo "  run          - Run the project in debug mode"
	@echo "  run-release  - Run the project in release mode"
	@echo "  clean        - Remove build artifacts"
	@echo "  check        - Check code for errors without building"
	@echo "  test         - Run tests"
	@echo "  fmt          - Format code"
	@echo "  check-csv    - Create a sample CSV file if none exists"
	@echo "  dist         - Create distribution files"
	@echo "  install      - Install the game (requires sudo)"
	@echo "  help         - Show this help message"
