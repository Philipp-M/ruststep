# Changelog

- All notable changes to this project will be documented in this file.
  - The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
  - and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

In addition to original Keep-a-Changelog, we use following rules:

- Use [GitHub Flavored Markdown](https://github.github.com/gfm/)
- Each line in changes SHOULD include a link to Pull Request in GitHub
- Each Pull Request MUST add a line in this file
  - This will be checked by GitHub Actions
- Each Pull Request MAY correspond to one or more lines in this file

## Unreleased (will be 0.2.0)

### Added
- Expose the module `derive_more` in `ruststep`.
- Snapshot testing for espr https://github.com/ricosjp/ruststep/pull/163
- espr_derive crate, `espr_derive::inline_express!` macro https://github.com/ricosjp/ruststep/pull/158
  - ruststep/tests uses `inline_express!` macro https://github.com/ricosjp/ruststep/pull/160
- Expose `ruststep_derive::*` macros in `ruststep::` namespace https://github.com/ricosjp/ruststep/pull/159
- Use rust-cache for faster CI https://github.com/ricosjp/ruststep/pull/156
- Comprehensive tests for ruststep_derive https://github.com/ricosjp/ruststep/pull/147
- Check CHANGELOG is updated in each pull request https://github.com/ricosjp/ruststep/pull/155
- `#[derive(Holder)]` for tuple struct https://github.com/ricosjp/ruststep/pull/146
- Test for `EntityTables` https://github.com/ricosjp/ruststep/pull/136
- Overview diagram written in asciiflow https://github.com/ricosjp/ruststep/pull/137
- `Tables` from `DataSection` https://github.com/ricosjp/ruststep/pull/139
- impl `FromStr` for `Record` and `DataSection` https://github.com/ricosjp/ruststep/pull/140

### Changed
- Translates declarations of type express to Rust tuple structure.
- Visitor struct and all fields in Holder struct become public https://github.com/ricosjp/ruststep/pull/160
- Add flag to switch ruststep internal/external codegen in IR::to_token_stream https://github.com/ricosjp/ruststep/pull/158
- Remove `ruststep_derive::as_holder_visitor!` https://github.com/ricosjp/ruststep/pull/147
- Use Rust 2021 edition https://github.com/ricosjp/ruststep/pull/128

### Fixed
- Drop unused derive_more, and dyn-clone crate dependencies https://github.com/ricosjp/ruststep/pull/159

### Deprecated
### Removed
### Security

## 0.1.0 - 2021-09-28

See https://github.com/ricosjp/ruststep/releases/tag/ruststep-0.1.0