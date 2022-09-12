#!/usr/bin/env bash

set -ex

cargo install --version 0.21.0 svd2rust
cargo install --version 0.8.0  form
rustup component add rustfmt
pip3 install --upgrade --user "svdtools==0.1.23"

rm -rf src
mkdir src

svd patch svd/rp2040.yaml

svd2rust -i svd/rp2040.svd.patched

form -i lib.rs -o src
rm lib.rs

cargo fmt

# Original svd has \n (two chars) in it, which gets converted to "\n" by svd2rust
# If we convert them to newline characters in the SVD, they don't turn up in markdown so docs suffers
# So, convert \n to [spc] [spc] [newline], then strip the spaces out if there are consecutive [newlines]
# This means that by the time we're in markdown \n\n becomes new paragraph, and \n becomes a new line
if [ "$(uname)" == "Darwin" ]; then
    find src -name '*.rs' -exec sed -i '' -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
else
    find src -name '*.rs' -exec sed -i -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
fi

# Sort specified fields alphanumerically for easier consumption in docs.rs
./sortFieldsAlphaNum.sh
