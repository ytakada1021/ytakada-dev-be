#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"/..

# Prerequisites
if [ ! -e .env.local ]; then
  echo Deploy failed.: .env.local shoud exist.
  exit 1
fi

# Read options.
WITH_BOOTSTRAP=0
HOTSWAP=0
while (($# > 0)); do
  case $1 in
  --with-bootstrap)
    WITH_BOOTSTRAP=1
    ;;
  --hotswap)
    HOTSWAP=1
    ;;
  *)
    echo Invalid option: $1. 1>&2
    exit 1
    ;;
  esac
  shift
done

# Build source.
./bin/build.sh

# Bootstrap if required.
if [ ${WITH_BOOTSTRAP} -eq 1 ]; then
  npm run cdklocal:bootstrap
fi

# Set env from env file.
export $(cat .env.local | grep -v "#" | xargs)

# Deploy to localstack (as hotswap if required).
case ${HOTSWAP} in
0)
  npm run cdklocal:deploy
  ;;
1)
  npm run cdklocal:hotswap
  ;;
*)
  This line should not be reached.
  exit 1
  ;;
esac
