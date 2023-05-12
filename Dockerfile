FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y curl

ENV tag=edk2-stable202302

RUN apt-get install -y \
    acpica-tools \
    g++ \
    gcc \
    git \
    make \
    nasm \
    python3 \
    uuid-dev

# Use git rather than a release tarball, since the tarball is missing
# submodules.
RUN git clone https://github.com/tianocore/edk2.git
WORKDIR /edk2
RUN git checkout "${tag}"
RUN git submodule update --init

RUN ./OvmfPkg/build.sh -a IA32 -a X64
