# Changelog

## [Unreleased]
* add `pull_request.yml` to check for added `CHANGELOG.md` entries on pull requests (#8)
* add `rust.yml` to run `cargo fmt`, `clippy`, `rustdoc`, and `test` for pull requests or pushes on `main` (#8)
* fix doc tests in `muddy` & ignore all doc tests in `muddy_macros` (not functional without `muddy`) (#9)
* remove Cargo.lock from repository [reason](https://blog.rust-lang.org/2023/08/29/committing-lockfiles.html) (#10)

## 0.2.0
* Initial published crate version
