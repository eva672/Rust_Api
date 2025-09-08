#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
TF_DIR="$ROOT_DIR/infra/terraform"

pushd "$TF_DIR" >/dev/null
terraform destroy -auto-approve -input=false || true
popd >/dev/null


