FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y curl

ENV tag=edk2-stable202302

# TODO
RUN apt-get install -y git

# Use git rather than a release tarball, since the tarball is missing
# submodules.
RUN git clone https://github.com/tianocore/edk2.git
WORKDIR /edk2
RUN git checkout "${tag}"
RUN git submodule update --init

# WORKDIR /edk2
# RUN curl -L -O \
#     "https://github.com/tianocore/edk2/archive/refs/tags/edk2-${release}.tar.gz"

# RUN tar xzf "edk2-${release}.tar.gz"

# WORKDIR "edk2-edk2-${release}"

# TODO
#RUN apt-get install -y patch

# TODO
#ADD patches patches

# Disable Brotli. It causes build errors, and we don't need it anyway.
# TODO: license for these patches
# RUN patch -p1 -i \
#     patches/0001-BaseTools-do-not-build-BrotliCompress-RH-only.patch

# RUN patch -p1 -i \
#     patches/0002-MdeModulePkg-remove-package-private-Brotli-include-p.patch

# TODO
RUN apt-get install -y \
    gcc \
    g++ \
    make
RUN apt-get install -y \
    python3 \
    uuid-dev

RUN apt-get install -y \
    acpica-tools \
    nasm

RUN ./OvmfPkg/build.sh -a X64

