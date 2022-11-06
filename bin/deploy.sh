#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"/..

# Prerequisites
if [ ! -e .env.prod ]; then
  echo Deploy failed.: .env.prod shoud exist.
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
./bin/build.sh --release

# Bootstrap if required.
if [ ${WITH_BOOTSTRAP} -eq 1 ]; then
  npm run cdk:bootstrap
fi

# Set env.
export $(cat .env.prod | grep -v "#" | xargs)
export TARGET=release

# Deploy to localstack (as hotswap if required).
case ${HOTSWAP} in
0)
  npm run cdk:deploy
  ;;
1)
  npm run cdk:hotswap
  ;;
*)
  This line should not be reached.
  exit 1
  ;;
esac
