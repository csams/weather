#!/usr/bin/env bash

set -e

./download-state.sh
./download-place.sh
./merge.py
