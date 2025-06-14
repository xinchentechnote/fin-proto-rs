# Makefile for fin-protoc project

# Variables
PROTO_DSL := proto/szse_bin_v1.29.dsl
PROTO_DIR := proto/
OUTPUT_DIR := src/
BIN_DIR := ~/workspace/fin-protoc/bin/

.PHONY: all compile format fix clean

all: compile fmt test

build: compile

compile:
	@echo "Compiling protocol..."
	$(BIN_DIR)/fin-protoc compile -f $(PROTO_DSL) -l $(PROTO_DIR) -r $(OUTPUT_DIR)

fmt: fix
	@echo "Formatting code..."
	cargo fmt
	$(BIN_DIR)/fin-protoc format -f $(PROTO_DSL)

fix:
	@echo "Applying fixes..."
	cargo fix --allow-dirty --allow-staged

test:
	@echo "Running tests..."
	cargo nextest run

# Help target
help:
	@echo "Available targets:"
	@echo "  all       - Run compile, format and fix (default)"
	@echo "  compile   - Compile the protocol definitions"
	@echo "  fmt       - Format the code using cargo fmt"
	@echo "  fix       - Apply automatic fixes using cargo fix"
	@echo "  help      - Show this help message"
