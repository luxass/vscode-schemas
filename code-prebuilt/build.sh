#!/usr/bin/env bash
# shellcheck disable=SC1091,SC2129


case "${OSTYPE}" in
  darwin*)
    export OS_NAME="osx"
    ;;
  msys* | cygwin*)
    export OS_NAME="win"
    ;;
  *)
    export OS_NAME="linux"
    ;;
esac

UNAME_ARCH=$(uname -m)

if [[ "${UNAME_ARCH}" == "arm64" ]]; then
  export VSCODE_ARCH="arm64"
elif [[ "${UNAME_ARCH}" == "ppc64le" ]]; then
  export VSCODE_ARCH="ppc64le"
else
  export VSCODE_ARCH="x64"
fi

if [[ -z "${VSCODE_TAG}" ]]; then
  echo "Retrieving latest version"
  LATEST_VERSION=$(curl --silent --fail "https://update.code.visualstudio.com/api/update/darwin/stable/0000000000000000000000000000000000000000")

  # print LATEST VERSION if debug is enabled
  if [[ "${DEBUG}" == "yes" ]]; then
    echo "LATEST_VERSION=${LATEST_VERSION}"
  fi

  TAG=$(echo "${LATEST_VERSION}" | jq -r '.name')
  COMMIT=$(echo "${LATEST_VERSION}" | jq -r '.version')

  mkdir -p vscode
  cd vscode || { echo "'vscode' dir not found"; exit 1; }

  git init -q
  git remote add origin https://github.com/microsoft/vscode.git

  git fetch --depth 1 origin "${COMMIT}"
  git checkout FETCH_HEAD
fi
