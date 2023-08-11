# Change Log

## [1.1.0] - 2023-08-11

CI (Continuos Integration) Docker image version : 1.1.0

### Changed
- Rust *xlsxwriter* package replaced with *rust_xlsxwriter* ([Cargo.toml](Cargo.toml)).
- Rust *chrono* package replaced with *time* ([Cargo.toml](Cargo.toml)).
### Removed
- *Clang* removed from [Dockerfile](.circleci/Dockerfile) as its not needed anymore.
