# Raiti

Touch typing tutor. Rust + [iced](https://github.com/iced-rs/iced) GUI. Lessons and keyboard layouts are YAML data files.

## Testing

Try create test for each feature you create

## Required checks for every change

Run all three after each change, before reporting it done. Do not skip any of them, even for a one-line edit.

```
cargo fmt
cargo clippy --all-targets -- -W clippy::pedantic
cargo build
```

## Commit rulles

Commit should be atomic and done as conventional commits
Write small commit messages and do not mention AI attributes there.
