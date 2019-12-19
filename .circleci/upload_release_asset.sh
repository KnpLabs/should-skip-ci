#!/bin/bash

set -o nounset
set -o errexit
set -o pipefail
[[ "${TRACE:-}" != "" ]] && set -o xtrace

printerr() {
    echo "${@}" >&2
}

get_assets_url() {
    url="$(curl -s \
        -u "knpEdgar:${GH_ACCESS_TOKEN}" \
        "https://api.github.com/repos/KnpLabs/should-skip-ci/releases/tags/${GIT_TAG}" \
    | jq -r .assets_url)"

    echo "${url/api.github.com/uploads.github.com}"
}

upload_bin() {
    local assets_url="${1}"

    curl -s \
        -u "knpEdgar:${GH_ACCESS_TOKEN}" \
        -H "Content-Type: application/octet-stream" \
        --data-binary "@${SSC_BIN}" \
        "${assets_url}?name=ssc-x86_64"
}


main () {
    if [[ -z "${GIT_TAG:-}" ]] || [[ -z "${SSC_BIN:-}" ]]; then
        printerr "Usage: GIT_TAG=vX.Y.Z SSC_BIN=path/to/ssc-bin upload_release_asset.sh"
        exit 1
    fi

    local assets_url="$(get_assets_url)"

    upload_bin "${assets_url}"
}

 main
