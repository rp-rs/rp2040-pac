#!/usr/bin/env bash

# Path to `svd`/`svdtools`
SVDTOOLS="${SVDTOOLS:-svdtools}"

set -ex

cargo install --version 0.33.4 svd2rust
cargo install --version 0.11.1  form
rustup component add rustfmt
if [ "$SVDTOOLS" == "svdtools" ]; then
    cargo install --version 0.3.12 svdtools
else
    python3 -mvenv --clear .venv
    source .venv/bin/activate
    pip3 install --upgrade "svdtools==0.1.25"
fi

$SVDTOOLS patch svd/rp2040.yaml

if [ "$SVDTOOLS" != "svdtools" ]; then
    deactivate
fi

rm -rf src
mkdir src

svd2rust -i svd/rp2040.svd.patched --reexport-core-peripherals --reexport-interrupt --ident-formats-theme legacy

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
