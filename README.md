# rust-re-export-dependencies

Investigation for usage of "Re-exporting"

## Directory

```
.
├── crates
│   ├── no-reexport // crate with old version "image", no re-exporting
│   ├── reexport-dependent-crate // crate with old version "image", re-exporting "image::DynamicImage"
│   └── reexport-dependent-types // crate with old version "image", re-exporting "image"
├── src
│   └── bin // executables using new version "image" and crates in ./crates/*
│       ├── main-no-reexport.rs
│       ├── main-reexport-dependent-crate.rs
│       └── main-reexport-dependent-types.rs
├── target
├── Cargo.lock
├── Cargo.toml
├── LICENSE
└── README.md
```
