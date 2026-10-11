#!/usr/bin/env bash
# Builds the Dioxus UI (in ui/) with every optimisation and copies it into wwwroot/,
# which is committed and baked into the docker image by the Dockerfile. Run this on
# the Mac mini after changing anything under ui/ or rest-api-shared/, then commit
# the updated wwwroot/. Just a convenience wrapper so you don't have to cd into ui/.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

exec "${SCRIPT_DIR}/ui/build.sh"
