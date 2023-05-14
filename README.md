# ovmf-prebuilt

Builds a release tag of [edk2](https://github.com/tianocore/edk2) and uploads it as a Github release to this repo.

The contents of the release tarball look like this:
```
x64/shell.efi
x64/vars.fd
x64/code.fd
ia32/shell.efi
ia32/vars.fd
ia32/code.fd
aarch64/vars.fd
aarch64/code.fd
aarch64/shell.efi
```

[**Latest Release**](https://github.com/rust-osdev/ovmf-prebuilt/releases/latest)

## License

OVMF is part of the [tianocore/edk2](https://github.com/tianocore/edk2) project. See the [`License.txt`](https://github.com/tianocore/edk2/blob/master/License.txt) and the [OVMF wiki page](https://github.com/tianocore/tianocore.github.io/wiki/OVMF) of the repository for licensing information. These are the licensing terms that applies to the releases in this repository. Note that some of the OVMF builds include a seabios CSM, which is GPLv3 licensed.

The _build code_ (not the releases) of this `ovmf-prebuilt` project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

