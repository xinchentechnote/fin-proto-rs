export PATH=~/workspace/fin-protoc/bin/:$PATH
fin-protoc compile -f proto/sse_bin_v0.57.dsl -o src/
cargo fix --allow-dirty --allow-staged
cargo fmt
