#!/usr/bin/env bash
#
# Builds the UI with every optimisation and stages it in ../wwwroot.
#
#   ./build.sh            (or ./build-ui.sh from the repo root)
#
# Run it on the Mac mini, then commit wwwroot/. CI does not build the UI: the release
# workflow builds the server only and the Dockerfile copies wwwroot/ as it is - so the
# image carries exactly what the last run of this script produced.
#
# What makes the bundle small (the same recipe as mt-client / my-prop-trading-client):
#   - [profile.release] in Cargo.toml: opt-level 3, fat LTO, one codegen unit, panic=abort;
#   - full wasm-opt -Oz ([web.wasm_opt] in Dioxus.toml) - on big modules it takes more RAM
#     than a CI runner has, which is why the bundle is built here;
#   - --debug-symbols false: neither names nor debug info stay in the wasm.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP="my-logger-ui-client"
WWWROOT="${SCRIPT_DIR}/../wwwroot"
DX_OUT="${SCRIPT_DIR}/target/dx/${APP}/release/web/public"

# The bundle is committed and nothing downstream rebuilds it, so the dx that produced it
# is what deploys. It must match the dioxus version in Cargo.toml.
DX_VERSION="0.7.10"

# `|| true` because of pipefail: without dx on PATH the script would die right here,
# before saying why.
dx_version_installed="$( { dx --version 2>/dev/null || true; } | awk '{print $2}' )"
if [ "${dx_version_installed}" != "${DX_VERSION}" ]; then
    cat >&2 <<MSG
dx version mismatch: the UI is built with ${DX_VERSION}, you have ${dx_version_installed:-none}.

    cargo binstall dioxus-cli --version ${DX_VERSION} --force
    # or: cargo install dioxus-cli --version ${DX_VERSION} --locked --force
MSG
    exit 1
fi

cd "${SCRIPT_DIR}"

LOG="$(mktemp)"
trap 'rm -f "${LOG}"' EXIT

# dx never clears its output folder, so the bundles of every previous build pile
# up there under old hashes - and would all be copied into wwwroot below.
echo ">> cleaning ${DX_OUT}"
rm -rf "${DX_OUT}"

echo ">> dx build --release --web --debug-symbols false"
# --verbose: dx reports running wasm-opt at the debug level only
dx --verbose build --release --web --debug-symbols false --log-to-file "${LOG}"

# A failed wasm-opt does not fail dx: it ships the unoptimised module and goes on. The
# file log says whether wasm-opt ran and how it ended.
if grep -q "wasm-opt failed" "${LOG}"; then
    echo "ERROR: wasm-opt failed - the bundle is NOT optimised, wwwroot is left as it was:" >&2
    grep -A5 "wasm-opt failed" "${LOG}" >&2
    exit 1
fi

if ! grep -q "Running wasm-opt" "${LOG}"; then
    echo "ERROR: dx did not run wasm-opt - the bundle is NOT optimised, wwwroot is left as it was" >&2
    exit 1
fi

# build.rs writes public/assets/app.css from css/ during the build, but dx takes its list
# of assets before that and ships the PREVIOUS build's sheet. public/assets has what
# build.rs has just written.
for css in "${SCRIPT_DIR}"/public/assets/*.css; do
    cp "${css}" "${DX_OUT}/assets/$(basename "${css}")"
done

echo ">> cleaning ${WWWROOT}"
rm -rf "${WWWROOT}"
mkdir -p "${WWWROOT}"

echo ">> copying ${DX_OUT}/. -> ${WWWROOT}/"
cp -R "${DX_OUT}/." "${WWWROOT}/"

# A release wasm build leaves gigabytes in target/ and nothing reads them again. A failed
# build stops the script above, so a target worth looking into is kept.
echo ">> removing ${SCRIPT_DIR}/target"
rm -rf "${SCRIPT_DIR}/target"

WASM_BYTES="$(wc -c < "$(ls "${WWWROOT}"/assets/*.wasm)" | tr -d ' ')"
echo ">> wasm: ${WASM_BYTES} bytes, wwwroot: $(du -sh "${WWWROOT}" | cut -f1)"
echo ">> done - commit wwwroot/"
