#!/usr/bin/env bash

set -e
mkdir -p place

pushd place
wget --no-clobber -e robots=off -r -np -R "index.html" --accept-regex '.*\.zip' -nd https://www2.census.gov/geo/tiger/TIGER2022/PLACE/
popd
