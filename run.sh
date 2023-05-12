#!/bin/sh

set -euxo pipefail

container_cmd="$1"
container_tag=ovmf_prebuilt

# The next release, edk2-stable202302, requires QEMU version 8+ (or a
# stable release with dab30fbef389 backported), otherwise the OVMF boot
# will hang. See https://bugzilla.tianocore.org/show_bug.cgi?id=4250 for
# more info.
#
# Since version 8 is pretty new, stick with an older EDK2 release for now.
git_tag=edk2-stable202211

# Build everything.
${container_cmd} build -t "${container_tag}" --build-arg git_tag="${git_tag}" .

# Copy out the tarball from the image.
${container_cmd} run "${container_tag}" cat "${git_tag}"-bin.tar.xz > "${git_tag}"-bin.tar.xz
