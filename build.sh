#/bin/bash
cd binary-codec
cargo build
cargo fmt
cargo nextest run
cd binary-codec-macro
cargo build
cargo fmt
cargo nextest run
cd bjse-binary
make
cd -
cd risk-binary
make
cd -
cd sample-binary
make
cd -
cd szse-binary
make
cd -
cd sse-binary
make
cd -
