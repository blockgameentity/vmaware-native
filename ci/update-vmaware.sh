#!/usr/bin/env bash

set -euo pipefail

readonly ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly RELEASE_API_URL="https://api.github.com/repos/NotRequiem/VMAware/releases/latest"
readonly HEADER="$ROOT/vendor/vmaware.hpp"
readonly CHECKSUM_FILE="$ROOT/vendor/vmaware.sha256"
readonly DUMPER_SOURCE="$ROOT/ci/dump_techniques.cpp"
readonly GENERATOR="$ROOT/ci/generate_techniques.py"
readonly GITHUB_API_VERSION="2026-03-10"

if [[ -z "${CXX:-c++}" ]]; then
    echo "CXX must name a C++ compiler" >&2
    exit 1
fi

read -r -a cxx_command <<< "${CXX:-c++}"
if (( ${#cxx_command[@]} == 0 )); then
    echo "CXX must name a C++ compiler" >&2
    exit 1
fi

for required_command in curl sha256sum awk grep install tr mktemp git python3 rustfmt cargo; do
    command -v "$required_command" >/dev/null 2>&1 || {
        echo "required command not found: $required_command" >&2
        exit 1
    }
done
command -v "${cxx_command[0]}" >/dev/null 2>&1 || {
    echo "required command not found: ${cxx_command[0]}" >&2
    exit 1
}

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
    --connect-timeout 10
    --max-time 120
    --header "Accept: application/vnd.github+json"
    --header "X-GitHub-Api-Version: $GITHUB_API_VERSION"
    --header "User-Agent: vmaware-native-updater"
)

if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    api_curl_args+=(--header "Authorization: Bearer $GITHUB_TOKEN")
fi

curl "${api_curl_args[@]}" \
    --output "$tmpdir/release.json" \
    "$RELEASE_API_URL"

asset_info="$(
    python3 - "$tmpdir/release.json" <<'PY'
import json
import sys

path = sys.argv[1]
try:
    with open(path, encoding="utf-8") as release_file:
        release = json.load(release_file)
except (OSError, json.JSONDecodeError) as exc:
    raise SystemExit(f"failed to parse GitHub release metadata: {exc}")

assets = release.get("assets", []) if isinstance(release, dict) else []
matching_assets = [
    asset
    for asset in assets
    if isinstance(asset, dict)
    and asset.get("name") == "vmaware.hpp"
    and asset.get("state") == "uploaded"
]

if len(matching_assets) != 1:
    raise SystemExit(
        "latest release must contain exactly one uploaded asset named vmaware.hpp"
    )

asset = matching_assets[0]
url = asset.get("url")
digest = asset.get("digest")
if not isinstance(url, str) or not url:
    raise SystemExit("release asset is missing a download URL")
if not isinstance(digest, str) or not digest:
    raise SystemExit("release asset is missing a digest")

print(f"{url}\t{digest}")
PY
)"
IFS=$'\t' read -r asset_api_url release_digest <<<"$asset_info"

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
    --connect-timeout 10
    --max-time 120
    --header "Accept: application/octet-stream"
    --header "X-GitHub-Api-Version: $GITHUB_API_VERSION"
    --header "User-Agent: vmaware-native-updater"
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
    dumper_include_dir="$(dirname "$HEADER")"
else
    dumper_include_dir="$tmpdir"
fi

"${cxx_command[@]}" -std=c++20 -I"$dumper_include_dir" "$DUMPER_SOURCE" \
    -o "$tmpdir/dump_techniques"
"$tmpdir/dump_techniques" > "$tmpdir/techniques.tsv"

python3 -c 'import pathlib, sys; source = pathlib.Path(sys.argv[1]); compile(source.read_text(encoding="utf-8"), str(source), "exec")' "$GENERATOR"
python3 "$GENERATOR" "$tmpdir/techniques.tsv"

if [[ "$release_checksum" != "$expected_checksum" ]]; then
    install -m 0644 "$tmpdir/vmaware.hpp" "$HEADER"
    printf '%s\n' "$upstream_checksum" > "$CHECKSUM_FILE"
fi

test "$(sha256sum "$HEADER" | awk '{print $1}')" = "$upstream_checksum"
! grep -qE '%tech(_all)?%' "$ROOT/src/technique.rs"

(
    cd "$ROOT"
    rustfmt --check src/technique.rs
    cargo test --all-targets
    git diff --check -- \
        src/technique.rs \
        vendor/vmaware.sha256
)

if [[ "$release_checksum" == "$expected_checksum" ]]; then
    echo "validated vendored VMAware header and generated bindings ($upstream_checksum)"
else
    echo "updated vendored VMAware header to $upstream_checksum"
fi
