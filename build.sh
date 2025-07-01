#/bin/bash
cd binary-codec
cargo build
cargo fmt
cargo nextest run
cd sample-binary
make
cd -
cd szse-binary
make
cd -
cd sse-binary
make
cd -
