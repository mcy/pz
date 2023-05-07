#!/bin/sh

PROTOC_VERSION=22.3
PROTOC_HOST=https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}
PROTOC_ARCH=x86_64
if [ "$(uname -m)" = "aarch64" ]; then
  PROTOC_ARCH=aarch_64
fi

PROTOC_ZIP=protoc-${PROTOC_VERSION}-linux-${PROTOC_ARCH}.zip

curl -OL ${PROTOC_HOST}/${PROTOC_ZIP} && \
  unzip -o ${PROTOC_ZIP} -d /usr/local bin/protoc && \
  unzip -o ${PROTOC_ZIP} -d /usr/local 'include/*' && \
  rm -f ${PROTOC_ZIP}
