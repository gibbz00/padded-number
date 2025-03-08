# Contributing Guidelines

There are some things that should be kept in mind when contributing to this project.
- Commit messages must follow the [conventional commits](https://www.conventionalcommits.org) specification. This enables automated [CHANGELOG.md](CHANGELOG.md) generation by using [`git-cliff`](https://git-cliff.org).
- Each commit is should pass CI on its own.
- Bug fixes and feature additions should be accompanied by tests.

### Recommended developer tools:

##### LSPs

* [`typos-lsp`](https://github.com/tekumara/typos-lsp)
* [`taplo`](https://github.com/tamasfe/taplo)
* [`rust-analyzer`](https://github.com/rust-lang/rust-analyzer)
### Using pre-commit hooks

This project makes use of [`pre-commit`](https://pre-commit.com/) hooks. A `pre-commit install` is recommended once the repository has been cloned, unless it has been [auto-enabled](https://pre-commit.com/#automatically-enabling-pre-commit-on-repositories).

### Running tests

Pretty standard procedure:

```sh
cargo test
```

### Generating and opening documentation

```sh
cargo doc --no-deps --all-features --open
```

## Release (for maintainers)

1. Make sure CI is not failing.
2. Bump version in the workspace `Cargo.toml` file.
3. Finally:

```sh
# <release> should match [0-9]+.[0-9]+.[0-9]+
git commit -am "chore: release <release>"
git tag "<release>"
git cliff -o CHANGELOG.md
git push && git push --tags
```
