set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

toolchain := ''
trimmed_toolchain := trim(toolchain)

cargo := if trimmed_toolchain != "" {
    "cargo +" + trimmed_toolchain
} else {
    "cargo"
}

default:
    @just --list

tidy: clippy fmt

clippy:
    {{cargo}} clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo +nightly fmt --all

check:
    {{cargo}} check --workspace --all-targets --all-features

build *extra_args:
    {{cargo}} build --workspace --all-targets --all-features {{extra_args}}

test *extra_args:
    {{cargo}} test --workspace --all-features {{extra_args}}

tarpaulin *extra_args:
    {{cargo}} tarpaulin --target-dir target-tarpaulin {{extra_args}}
    {{ if env('CI', '') == '' { `open tarpaulin-report.html` } else { ` ` } }}

reset-manifest-before-msrv:
    {{ if path_exists("Cargo.toml.bak.msrv") == "true" { `cp Cargo.toml.bak.msrv Cargo.toml` } else { ` ` } }}

check-min-versions: reset-manifest-before-msrv
    {{cargo}} minimal-versions check --workspace --lib --bins --all-features

backup-manifest-before-msrv:
    cp Cargo.toml Cargo.toml.bak.msrv

backup-lockfile-before-msrv:
    {{ if path_exists("Cargo.lock") == "true" { `cp Cargo.lock Cargo.lock.bak.msrv` } else { ` ` } }}

unset-rust-version:
    -toml unset --toml-path Cargo.toml package.rust-version

restore-msrv-manifest:
    mv Cargo.toml.bak.msrv Cargo.toml

restore-msrv-lockfile:
    {{ if path_exists("Cargo.lock.bak.msrv") == "true" { `mv Cargo.lock.bak.msrv Cargo.lock` } else { `rm Cargo.lock` } }}

msrv: backup-manifest-before-msrv backup-lockfile-before-msrv unset-rust-version && restore-msrv-manifest restore-msrv-lockfile
    cargo msrv -- just check-min-versions

update-to-min-versions: backup-manifest-before-msrv backup-lockfile-before-msrv && restore-msrv-manifest
    cargo hack --remove-dev-deps --workspace
    cargo +nightly update -Z minimal-versions

doc $RUSTDOCFLAGS="-D warnings":
    {{cargo}} doc {{ if env('CI', '') != '' { '--no-deps' } else { '--open' } }} --workspace --all-features

doc-coverage $RUSTDOCFLAGS="-Z unstable-options --show-coverage":
    cargo +nightly doc --no-deps --workspace --all-features

test-package:
    {{cargo}} publish --dry-run
