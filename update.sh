#!/usr/bin/env bash

set -ex

# NOTE: Last executed using Rust 1.49.0

cargo install --version 0.17.0 svd2rust
cargo install --version 0.7.0  form
rustup component add rustfmt
pip3 install --upgrade --user "svdtools>=0.1.13"

rm -rf src
mkdir src

svd patch svd/rp2040.yaml

svd2rust -i svd/rp2040.svd.patched

form -i lib.rs -o src
rm lib.rs

cargo fmt

sed -i "/extern crate bare_metal;/d" ./src/lib.rs
sed -i 's/bare_metal::Nr/cortex_m::interrupt::Nr/g' ./src/lib.rs

# Sort specified fields alphanumerically for easier consumption in docs.rs
./sortFieldsAlphaNum.sh
