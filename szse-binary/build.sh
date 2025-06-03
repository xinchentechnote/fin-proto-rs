export PATH=~/workspace/fin-protoc/bin/:$PATH
fin-protoc compile -f proto/szse_bin_v1.29.dsl -o src/
cargo fix --allow-dirty --allow-staged
cargo fmt
