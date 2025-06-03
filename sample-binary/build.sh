export PATH=~/workspace/fin-protoc/bin/:$PATH
fin-protoc compile -f proto/sample_binary.dsl -o src/
cargo fix --allow-dirty --allow-staged
cargo fmt
