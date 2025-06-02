export PATH=~/workspace/fin-protoc/bin/:$PATH
fin-protoc compile -f sample-binary/proto/sample_binary.dsl -o sample-binary/src/
cargo fix --allow-dirty --allow-staged
cargo fmt
