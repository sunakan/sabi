#!/bin/sh

set -eu

for dir in $(find . -maxdepth 1 -mindepth 1 -type d -not -iwholename ./.git); do
  cd $dir
  cargo fmt
  cd ../
done
