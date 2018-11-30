#! /bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
${DIR}/generate.sh && \
${DIR}/lint.sh && \
${DIR}/build.sh
echo "Publishing..."