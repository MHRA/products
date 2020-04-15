# Developing and Releasing this Package

## Release Candidates

To publish a new, release candidate version of this package:

1. Set a new version number in `Cargo.toml` with an `-rc` suffix (for example, `1.2.2` is incremented to `1.2.3-rc1`, `1.2.3-rc4` is incremented to `1.2.3-rc5`).
2. Run `cargo publish` to publish the release candidate.
3. Make use of the release candidate!

## Publishing a Stable Release

1. Remove `-rc` from the version number in `Cargo.toml` (for example, `1.2.3-rc4` is incremented to `1.2.3`).
2. Run `cargo publish`.
3. Make use of the stable version!