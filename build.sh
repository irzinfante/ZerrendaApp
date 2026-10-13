#!/usr/bin/env bash

echo -n "Build app.. "

cd ./app 2>/dev/null || { echo "Error!"; echo "Folder ./app not found"; exit 1; }

needs_install() {
  [ ! -d node_modules ] && return 0
  [ package.json -nt node_modules ] && return 0
  [ -f package-lock.json ] && [ package-lock.json -nt node_modules ] && return 0
  return 1
}

if output=$(
  (
    set -e
    if needs_install; then
      npm install
      touch node_modules
    fi
    npm run build
    
  ) 2>&1
); then
  echo "Ok!"
else
  echo "Error!"
  echo "$output"
  exit 1
fi

cd ..

echo -n "Build api.. "

cd ./api 2>/dev/null || { echo "Error!"; echo "Folder ./api not found"; exit 1; }

if output=$(cargo build 2>&1); then
  echo "Ok!"
else
  echo "Error!"
  echo "$output"
  exit 1
fi