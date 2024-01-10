set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

toolchain := ''
trimmed_toolchain := trim(toolchain)

cargo := if trimmed_toolchain != "" {
    "cargo +" + trimmed_toolchain
} else {
    "cargo"
}

all_features := "true"
all_features_flag := if all_features == "true" { "--all-features" } else { "" }

all_targets := "true"
all_targets_flag := if all_targets == "true" { "--all-targets" } else { "" }

default:
    @just --list

tidy: clippy fmt

clippy:
    {{cargo}} clippy --workspace {{all_targets_flag}} {{all_features_flag}} -- -D warnings

fmt:
    cargo +nightly fmt --all

check *extra_args:
    {{cargo}} check --workspace {{all_targets_flag}} {{all_features_flag}} {{extra_args}}

build *extra_args:
    {{cargo}} build --workspace {{all_targets_flag}} {{all_features_flag}} {{extra_args}}

test *extra_args:
    {{cargo}} test --workspace {{all_features_flag}} {{extra_args}}

tarpaulin *extra_args:
    {{cargo}} tarpaulin --target-dir target-tarpaulin {{extra_args}}
    {{ if env('CI', '') == '' { `open tarpaulin-report.html` } else { ` ` } }}

doc $RUSTDOCFLAGS="-D warnings":
    {{cargo}} doc {{ if env('CI', '') != '' { '--no-deps' } else { '--open' } }} --workspace {{all_features_flag}}

doc-coverage $RUSTDOCFLAGS="-Z unstable-options --show-coverage":
    cargo +nightly doc --no-deps --workspace {{all_features_flag}}

minimize:
    cargo hack --remove-dev-deps --workspace
    cargo +nightly update -Z minimal-versions

check-minimal:
    {{ if path_exists("Cargo.toml") == "true" { `mv Cargo.toml Cargo.toml.bak` } else { ` ` } }}
    {{ if path_exists("Cargo.lock") == "true" { `mv Cargo.lock Cargo.lock.bak` } else { ` ` } }}
    cp Cargo.toml.msrv Cargo.toml
    {{cargo}} minimal-versions check --workspace --lib --bins {{all_features_flag}}
    {{ if path_exists("Cargo.toml.bak") == "true" { `mv Cargo.toml.bak Cargo.toml` } else { `rm Cargo.toml` } }}
    {{ if path_exists("Cargo.lock.bak") == "true" { `mv Cargo.lock.bak Cargo.lock` } else { `rm Cargo.lock` } }}

msrv:
    cargo msrv -- just check-minimal

test-package:
    {{cargo}} publish --dry-run
