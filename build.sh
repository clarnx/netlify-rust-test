#!/bin/bash
# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

cargo zigbuild --target x86_64-unknown-linux-gnu.2.26 --release
mkdir -p functions
cp target/x86_64-unknown-linux-gnu/release/hello functions/
cp target/x86_64-unknown-linux-gnu/release/current-time functions/
cp target/x86_64-unknown-linux-gnu/release/axum-lambda-demo functions/