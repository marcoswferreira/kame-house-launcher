# Contributing Guide: Steam Companion 🐢

Thank you for your interest in contributing to **Steam Companion**! This document provides guidelines for development workflows, code standards, and commit practices.

<p align="center">
  <b>English</b> · <a href="CONTRIBUTING.pt-BR.md">Português (Brasil)</a>
</p>

---

## 🌟 Core Principles

1. **Idiomatic, Memory-Safe Rust:** Emphasize type safety, strict error handling via `anyhow`/`thiserror`, and minimal heap allocations in inner loops.
2. **High Performance & Low Overhead:** The launcher must maintain a lightweight background footprint (~40MB RAM) and keep hardware extensions opt-in so it never competes with games or emulators.
3. **Mandatory Test Coverage (≥ 70%):** Every new feature, parser, or module must include unit or integration tests, maintaining the workspace line coverage at or above 70%.

---

## 🛠 Development Setup

1. Ensure the Rust toolchain is installed:

   ```bash
   rustup update stable
   rustup component add llvm-tools-preview
   ```

2. Install development utilities:

   ```bash
   cargo install cargo-llvm-cov --locked
   ```

3. Clone repository recursively:

   ```bash
   git clone --recurse-submodules https://github.com/marcoswferreira/kame-house-launcher.git
   cd kame-house-launcher
   ```

---

## 🧪 Local Testing & Verification

Before submitting Pull Requests or creating commits:

1. **Check Formatting and Lints:**

   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   ```

2. **Run Workspace Tests:**

   ```bash
   cargo test --workspace
   ```

3. **Check Code Coverage:**

   ```bash
   cargo llvm-cov --workspace --summary-only
   ```

   Total line coverage must remain above 70%.

---

## 📝 Commit Standards (Conventional Commits)

All commits must follow the [Conventional Commits v1.0.0](https://www.conventionalcommits.org/) specification:

```text
<type>(<optional scope>): <imperative summary in lowercase>

[optional body explaining motivation and changes]

[optional footer for issue references or breaking changes]
```

### Allowed Types

- `feat`: New user-facing feature or library capability.
- `fix`: Bug fix.
- `docs`: Documentation or ADR updates only.
- `style`: Formatting, whitespace, no production logic changes.
- `refactor`: Code change that neither fixes a bug nor adds a feature.
- `perf`: Performance optimization.
- `test`: Adding missing tests or correcting existing tests.
- `chore`: Build scripts, dependencies, or CI/CD updates.

### Examples of Good Commit Messages

- `feat(core): add rom header detection for sega saturn disc images`
- `fix(steam): correct vdf binary endianness when writing 64-bit integers`
- `docs: update dualsense pairing instructions in readme`
- `test(console): add unit tests for display resolution presets`

---

## 🔀 Branching & Pull Request Workflow

1. Create a descriptive feature branch from `main`:

   ```bash
   git checkout -b feat/gamepad-virtual-keyboard
   ```

2. Make atomic, well-documented commits.
3. Ensure all tests and coverage checks pass locally.
4. Open a Pull Request detailing the problem solved and testing performed.
