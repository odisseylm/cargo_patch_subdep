# cargo-patch-subdep

`Cargo-Patch-SubDependencies` is a Cargo Subcommand which allows patching sub-dependencies versions.
It gets them already downloaded from std cargo registry, puts to './target/patch-override-sub-dep'
and adds patch entry to root manifest Cargo.toml file.
Downloading dependency sources are done by creating temporary empty project with
required dependencies section and using 'cargo' crate to resolve it and get 
path to downloaded dependency sources.


## Installation

Simply run:

```sh
cargo install mvv_cargo_patch_subdep_ver
```
```sh
cargo install mvv_cargo_patch_subdep_ver --path /home/...bla-bala.../cargo_patch_subdep
```
```sh
cargo install mvv_cargo_patch_subdep_ver --path .
```
```sh
cargo install --git https://github.com/odisseylm/cargo_patch_subdep
```


Using only 'build.rs' is not supported now.
I don't know how to register hook before manifest check/resolve action.
'build.rs' is called after and if manifest already has patch entries
but (patched/not-patched) sources are not put yet into './target/patch-override-sub-dep'
resolve/check fails and 'build.rs' is not called.


## Usage

To patch a dependency one has to add the following to `Cargo.toml`:

```toml
[workspace.metadata.patch-replace-sub-dependencies.progenitor]
replace = [ { sub_dep = "reqwest", from_ver = "0.11.27", to_ver = "0.12.5" }, ]
ignore_cargos = [ "third-party/some-crate-to-skip/" ] # With current simple impl this 'skip' will be applied to ALL entries.
[workspace.metadata.patch-replace-sub-dependencies.progenitor-impl]
replace = [ { sub_dep = "reqwest", from_ver = "0.11.27", to_ver = "0.12.5" }, ]
ignore_cargos = [ "third-party/some-crate-to-skip/" ]
[workspace.metadata.patch-replace-sub-dependencies.progenitor-client]
replace = [ { sub_dep = "reqwest", from_ver = "0.11.27", to_ver = "0.12.5" }, ]
ignore_cargos = [ "third-party/some-crate-to-skip/" ]
```

It specifies which dependency to patch (in this case 'progenitor-xxx').
Running:

```sh
cargo patch-subdep-ver
```

will download the packages specified in the dependency section to the
'./target/patch-override-sub-dep' folder and change sub-dependency versions.
And the similar path entries will be added into root 'Cargo.toml'.

```toml
[patch.crates-io.progenitor]
version = "0.7.0"
path = "target/patch-override-sub-dep/progenitor/progenitor-0.7.0"

[patch.crates-io.progenitor-impl]
version = "0.7.0"
path = "target/patch-override-sub-dep/progenitor-impl/progenitor-impl-0.7.0"

[patch.crates-io.progenitor-client]
version = "0.7.0"
path = "target/patch-override-sub-dep/progenitor-client/progenitor-client-0.7.0"
```


## Limitations

??? It's only possible to patch dependencies of binary crates as it is not possible
for a subcommand to intercept the build process.
