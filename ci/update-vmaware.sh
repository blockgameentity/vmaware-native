#!/usr/bin/env bash

set -euo pipefail

readonly ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly RELEASE_API_URL="https://api.github.com/repos/kernelwernel/VMAware/releases/latest"
readonly HEADER="$ROOT/vendor/vmaware.hpp"
readonly CHECKSUM_FILE="$ROOT/vendor/vmaware.sha256"
readonly DUMPER_SOURCE="$ROOT/ci/dump_techniques.cpp"
readonly GENERATOR="$ROOT/ci/generate_techniques.py"

for command in curl jq sha256sum awk grep install tr mktemp git python3 "${CXX:-c++}" rustfmt cargo; do
    command -v "$command" >/dev/null 2>&1 || {
        echo "required command not found: $command" >&2
        exit 1
    }
done

expected_checksum="$(tr -d '[:space:]' < "$CHECKSUM_FILE")"
current_checksum="$(sha256sum "$HEADER" | awk '{print $1}')"

if [[ ! "$expected_checksum" =~ ^[0-9a-f]{64}$ ]]; then
    echo "invalid checksum in $CHECKSUM_FILE" >&2
    exit 1
fi

if [[ "$current_checksum" != "$expected_checksum" ]]; then
    echo "$HEADER does not match $CHECKSUM_FILE" >&2
    exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

api_curl_args=(
    --fail
    --silent
    --show-error
    --location
    --retry 3
    --retry-all-errors
    --header "Accept: application/vnd.github+json"
    --header "X-GitHub-Api-Version: 2026-03-10"
)

if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    api_curl_args+=(--header "Authorization: Bearer $GITHUB_TOKEN")
fi

curl "${api_curl_args[@]}" \
    --output "$tmpdir/release.json" \
    "$RELEASE_API_URL"

asset_json="$(
    jq -cer '
        [.assets[] | select(.name == "vmaware.hpp" and .state == "uploaded")]
        | if length == 1 then .[0]
          else error("latest release must contain exactly one uploaded asset named vmaware.hpp")
          end
    ' "$tmpdir/release.json"
)"
asset_api_url="$(jq -er '.url | select(type == "string" and length > 0)' <<<"$asset_json")"
release_digest="$(jq -er '.digest | select(type == "string")' <<<"$asset_json")"

if [[ ! "$release_digest" =~ ^sha256:([0-9a-f]{64})$ ]]; then
    echo "invalid or missing SHA-256 digest for release asset vmaware.hpp" >&2
    exit 1
fi

release_checksum="${BASH_REMATCH[1]}"
download_curl_args=(
    --fail
    --silent
    --show-error
    --location
    --retry 3
    --retry-all-errors
    --header "Accept: application/octet-stream"
    --header "X-GitHub-Api-Version: 2026-03-10"
)

if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    download_curl_args+=(--header "Authorization: Bearer $GITHUB_TOKEN")
fi

curl "${download_curl_args[@]}" \
    --output "$tmpdir/vmaware.hpp" \
    "$asset_api_url"

upstream_checksum="$(sha256sum "$tmpdir/vmaware.hpp" | awk '{print $1}')"

if [[ "$upstream_checksum" != "$release_checksum" ]]; then
    echo "downloaded vmaware.hpp digest does not match release digest" >&2
    exit 1
fi

if [[ "$release_checksum" == "$expected_checksum" ]]; then
    echo "vendored VMAware header is already current ($upstream_checksum)"
    exit 0
fi

"${CXX:-c++}" -std=c++20 -I"$tmpdir" "$DUMPER_SOURCE" \
    -o "$tmpdir/dump_techniques"
"$tmpdir/dump_techniques" > "$tmpdir/techniques.tsv"

python3 -c 'compile(open("'"$GENERATOR"'", encoding="utf-8").read(), "'"$GENERATOR"'", "exec")'
python3 "$GENERATOR" "$tmpdir/techniques.tsv"

install -m 0644 "$tmpdir/vmaware.hpp" "$HEADER"
printf '%s\n' "$upstream_checksum" > "$CHECKSUM_FILE"

test "$(sha256sum "$HEADER" | awk '{print $1}')" = "$upstream_checksum"
! grep -qE '%tech(_all)?%' "$ROOT/src/technique.rs"

(
    cd "$ROOT"
    rustfmt --check src/technique.rs
    cargo test --all-targets
    git diff --check -- \
        src/technique.rs \
        vendor/vmaware.hpp \
        vendor/vmaware.sha256
)

echo "updated vendored VMAware header to $upstream_checksum"
