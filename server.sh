exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/rust-http-server-target \
    --manifest-path $(dirname $0)/Cargo.toml -- "$@"
