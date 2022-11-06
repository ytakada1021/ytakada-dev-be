#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"/..

# Read options.
STAGE="debug" # debug or release
while (($# > 0)); do
  case $1 in
  --release)
    STAGE="release"
    ;;
  *)
    echo "Invalid option: $1." 1>&2
    exit 1
    ;;
  esac
  shift
done

# Build
if [ ${STAGE} = debug ]; then
  echo Debug build
  cargo build
else
  echo Release build
  cargo build --release
fi

# Copy
readonly BASE_DIR=./target/${STAGE}/deploy
readonly SAVE_POST_DIR=${BASE_DIR}/save-post
readonly DELETE_POST_DIR=${BASE_DIR}/delete-post
readonly GET_POST_DIR=${BASE_DIR}/get-post
readonly LIST_POST_DIR=${BASE_DIR}/list-post
mkdir -p ${SAVE_POST_DIR} ${DELETE_POST_DIR} ${GET_POST_DIR} ${LIST_POST_DIR}
cp ./target/${STAGE}/save_post ${SAVE_POST_DIR}/bootstrap
# cp ./target/${STAGE}/delete_post ${DELETE_POST_DIR}/bootstrap
# cp ./target/${STAGE}/get_post ${GET_POST_DIR}/bootstrap
# cp ./target/${STAGE}/list_post ${LIST_POST_DIR}/bootstrap
