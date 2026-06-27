#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.." 
AWS_REGION="${AWS_REGION:-ap-southeast-1}"

echo "Deploying with cargo-lambda..."
echo "AWS_REGION=${AWS_REGION}"

export AWS_REGION

echo "Compiling (cargo lambda build)..."
cargo lambda build --release --arm64
echo "Compilation complete."

cargo lambda deploy --profile owen
echo "Deploy complete."