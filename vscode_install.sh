#/bin/bash
set -eu
mkdir -p tmp
cargo snippet -t vscode > tmp/cargo_snippets.json
jq -s -M add vscode.json tmp/cargo_snippets.json > tmp/rust.json
echo UPDATE ${VSCODE_SNIPPET_PATH}/rust.json
cp tmp/rust.json ${VSCODE_SNIPPET_PATH}/rust.json