#!/usr/bin/env bash
# shellcheck disable=SC1091,2154

# copy license file
cp -f LICENSE vscode/LICENSE

cd vscode || { echo "'vscode' dir not found"; exit 1; }

# patch settings

# apply patches
{ set +x; } 2>/dev/null

for file in ../patches/*.patch; do
  if [[ -f "${file}" ]]; then
    echo applying patch: "${file}"
    # grep '^+++' "${file}"  | sed -e 's#+++ [ab]/#./vscode/#' | while read line; do shasum -a 256 "${line}"; done
    if ! git apply --ignore-whitespace "${file}"; then
      echo failed to apply patch "${file}" >&2
      exit 1
    fi
  fi
done

set -x

export ELECTRON_SKIP_BINARY_DOWNLOAD=1
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

if [[ "${OS_NAME}" == "linux" ]]; then
  export VSCODE_SKIP_NODE_VERSION_CHECK=1

   if [[ "${npm_config_arch}" == "arm" ]]; then
    export npm_config_arm_version=7
  fi

  CHILD_CONCURRENCY=1 yarn --frozen-lockfile --check-files --network-timeout 180000
elif [[ "${OS_NAME}" == "osx" ]]; then
  CHILD_CONCURRENCY=1 yarn --frozen-lockfile --network-timeout 180000

  yarn postinstall
else
  rm -rf .build/node-gyp
  mkdir -p .build/node-gyp
  cd .build/node-gyp

  git config --global user.email "$( echo "${GITHUB_USERNAME}" | awk '{print tolower($0)}' )-ci@not-real.com"
  git config --global user.name "${GITHUB_USERNAME} CI"
  git clone https://github.com/nodejs/node-gyp.git .
  git checkout v10.0.1
  npm install

  npm_config_node_gyp="$( pwd )/bin/node-gyp.js"
  export npm_config_node_gyp

  cd ../..

  if [[ "${npm_config_arch}" == "arm" ]]; then
    export npm_config_arm_version=7
  fi

  CHILD_CONCURRENCY=1 yarn --frozen-lockfile --check-files --network-timeout 180000
fi
