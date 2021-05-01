#!/bin/sh
set -eu

readonly GITIGNORE_URL=https://www.toptal.com/developers/gitignore/api/linux,macos,windows,vim,emacs,jetbrains+all,rust
curl -o .gitignore ${GITIGNORE_URL}

for dir in $(find . -maxdepth 1 -mindepth 1 -type d -not -iwholename ./.git); do
  echo $dir
  cp .gitignore ${dir}/
done

rm .gitignore
