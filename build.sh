#/bin/bash
cd binary-codec
cargo build
cargo fmt
cargo nextest run
cd sample-binary
make build
make fmt
make test
cd -
cd szse-binary
make build
make fmt
make test
cd -
cd sse-binary
make build
make fmt
make test
cd -
