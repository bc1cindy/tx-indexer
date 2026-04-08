# Contributing

---

Welcome to the Transaction Graph Analysis Framework (tx-indexer).

This project implements an analysis framework for the Bitcoin transaction graph that evaluates the privacy properties of both simulated and real-world privacy protocols.

Contributions are greatly valued and impactful: whether it's reporting issues, writing documentation, implementing new heuristics, or contributing code, we'd love your help!

---

## Communication Channels

Most discussion about transaction graph analysis research and development happens on [Discord](https://discord.gg/6qnwB6Qq), or in Github [issues](https://github.com/payjoin/tx-indexer/issues) or [pull requests](https://github.com/payjoin/tx-indexer/pulls).

---

## Issues

Using and testing the tx-indexer framework is an effective way for new contributors to both learn and provide value. If you find a bug, incorrect or unclear documentation, or have any other problem, consider [creating an issue](https://github.com/payjoin/tx-indexer/issues). Before doing so, please search through [existing issues](https://github.com/payjoin/tx-indexer/issues) to see if your problem has already been addressed or is actively being discussed. If you can, provide a fully reproducible example or the steps we can use to reproduce the issue to speed up the debugging process.

---

## Documentation

Good documentation is essential to understanding what tx-indexer does and how to use it. Since tx-indexer aims to make chain analysis methods open source and accessible to the broader research and development community, providing clear and complete documentation is critical. Good documentation is also invaluable to new contributors ramping up quickly. If _you_ find something hard to understand or difficult to figure out how to use from the documentation, it's a sign they could be improved. To contribute to the documentation please [fork the repository](https://github.com/payjoin/tx-indexer/fork), make changes there, and then submit a pull request.

---

## Code

### Getting Started

If you're looking for somewhere to start contributing code changes, see the [good first issue](https://github.com/payjoin/tx-indexer/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22) list. If you intend to start working on an issue, please leave a comment stating your intent.

To contribute a code change:

1. [Fork the repository](https://github.com/payjoin/tx-indexer/fork).
2. Create a topic branch.
3. Commit changes.

### Commits

The git repository is our source of truth for development history. Therefore the commit history is the most important communication
artifact we produce. Commit messages must follow [the seven rules in this guide by cbeams](https://cbea.ms/git-commit/#seven-rules).

Every commit should be [hygenic](https://github.com/bitcoin/bitcoin/blob/master/CONTRIBUTING.md#committing-patches) and pass CI. This means tests, linting, and formatting should pass without issues on each commit. Below is a [git hook](https://git-scm.com/book/ms/v2/Customizing-Git-Git-Hooks) you may choose to add to `.git/hooks/pre-commit` in your local repository to perform these checks before each commit:

```sh
#!/usr/bin/env bash
set -euo pipefail

# -------- 1. Rustfmt --------
echo "▶  cargo fmt --check"
cargo fmt --all -- --check

# -------- 2.1 Project-specific linter --------
echo "▶  ./contrib/lint.sh"
./contrib/lint.sh

# -------- 2.2 Documentation builder --------
echo '▶  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --document-private-items'
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --document-private-items

# -------- 3. Fast test suite --------
echo "▶  ./contrib/test.sh"
./contrib/test.sh

echo "✓  Pre-commit hook passed"

```

### Testing

We test a few different features combinations in CI. To run all of the combinations locally, run `contrib/test.sh`.

If you are adding a new feature, heuristic, or analysis method, please add tests for it.

### CI Process

Our continuous integration runs on every pull request and push to master. The CI pipeline includes:

1. **Formatting check** - Ensures all code is properly formatted with `cargo fmt`
2. **Linting** - Runs `clippy` with all targets and features to catch common mistakes and enforce best practices
3. **Tests** - Runs the full test suite across different feature combinations
4. **Documentation** - Builds documentation to ensure no broken doc links or examples

All CI checks must pass before a PR can be merged. You can run these checks locally using the scripts in `contrib/` or the pre-commit hook shown above.

### Upgrading dependencies

If your change requires a dependency to be upgraded, please ensure `Cargo.lock` is updated before submitting any changes.

### Code Formatting

We use the stable Rust formatter for this project. Please run [`rustfmt`](https://github.com/rust-lang/rustfmt) before submitting any changes, or simply run `contrib/lint.sh` which handles formatting.

### Linting

We use [`clippy`](https://github.com/rust-lang/rust-clippy) for linting. Please run `contrib/lint.sh` before submitting any changes.
