# Agent instructions for material3-gpui

This file defines the expectations for all contributions (human and LLM‑assisted) to the `material3-gpui` repository.

---

## Project Overview

- **Repository**: https://github.com/TNTyep520/md3-gpui
- **Description**: Material Design 3 component library for the `gpui` framework (from Zed).
- **Main dependencies**: `gpui`, `anyhow` (see `Cargo.toml`).

---

## Code Style & Rust Conventions

All Rust code must follow the **standard Rust style** as enforced by `rustfmt` and `clippy`.

### Formatting

- Run `cargo fmt` before committing.
- Use `cargo clippy` with default lint set; fix or explicitly allow any warnings.

### Naming

- Follow Rust naming conventions: `snake_case` for variables, functions, modules; `CamelCase` for types, traits; `SCREAMING_SNAKE_CASE` for constants.
- Avoid abbreviations unless widely accepted (e.g., `len`).

### Nullability & Optional Values

- Use `Option<T>` to represent an optional value.
- Never use `null`, `NULL`, or C‑style null pointers.
- Use `Result<T, E>` for fallible operations; prefer `anyhow::Result` for application errors.
- Avoid `unwrap()`, `expect()`, `unwrap_unchecked()` in production code. Use `?` or proper error handling; document if unavoidable.

### Immutability

- Variables and data structures are immutable by default. Use `mut` only when necessary.
- Prefer `&[T]` or `&str` for read‑only views.
- For buffers (e.g., `&[u8]`, `ByteBuf`), treat as read‑only unless explicitly mutable.

### Types

- Use `struct` or `enum`; derive common traits (`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, etc.) when appropriate.
- Use type aliases for complex or repeated types.

### Imports and Fully Qualified Names

- **Do not use fully qualified names** in code. Always use `use` imports to bring items into scope.
- Organise imports according to Rust conventions (std, external, crate, super, self).

### Character Encoding and Line Endings

- All files must be saved with **UTF‑8 without BOM** encoding.
- When working on Windows with PowerShell, be mindful of **escape characters and BOM injection** that may inadvertently introduce corruption or formatting issues.

---

## Documentation

- Every public item **must** have a documentation comment (`///` or `//!`).
- Documentation must describe purpose, behaviour, constraints, and side effects.
- Add implementation comments (`//`) inside complex logic where clarity is improved.
- Include examples in doc comments for non‑trivial APIs.

---

## Dependency Management

- All dependencies are declared in `Cargo.toml`.
- Use `cargo add` to add new dependencies, or edit the file manually.
- Keep dependencies up‑to‑date; run `cargo update` regularly.

---

## Git Commit & PR Guidelines

- **Commit style**: strictly follow the existing commit message conventions of this repository (e.g., conventional commits, imperative tense, scope, etc.). If no explicit style is documented, emulate the previous commit history.
- Commits should be atomic and have clear, concise messages.
- **Do not** commit, push, or open a pull request **without explicit user confirmation**.
- **Do not** modify development environment settings (e.g., `.cargo/config`, `.vscode`, `.idea`, or local toolchain files) unless explicitly requested.
- For LLM‑assisted changes, the PR description **must** disclose the use of LLM (extent and purpose).
- Do **not** add `Co-Authored-By` trailers; disclosure is sufficient.

---

## Interaction Guidelines (for the Agent)

- **Language**: Prefer using **Chinese** when communicating with the user; technical terms may remain in English.
- **Requirement confirmation**: Before implementing any change, **fully and accurately confirm** the scope of the requirement. Do not assume or infer unstated details.
- **Ask questions**: Proactively ask the user about technical details until you have **deterministic information** to proceed.

---

## LLM Usage Policy (adapted from Rust project guidelines)

The following rules apply to all LLM‑generated content and actions.

### Prohibited Text Generation

- Do **not** generate or rewrite non‑trivial PR descriptions, issue bodies, public comments, user‑facing documentation, diagnostic messages, or source comments.
- If you need to change a message or comment, **stop** and ask the user to write the new text manually. You may then mechanically regenerate test snapshots if applicable.
- You may **explain** what the message should communicate, but must not provide paste‑ready wording.

### Gate‑Failure Protocol

If a rule identifies banned work, **stop immediately**. Do not offer alternatives, promise to continue later, or produce drafts. State the triggering rule and the required action.

### Reviewer Requirement

- Do not commit any LLM‑generated repository change unless the user has named, in this conversation, another person who agreed to review it.
- A general “review was solicited” is not sufficient; the reviewer must be explicitly named.
- This requirement does **not** apply to temporary debugging aids or local tooling that will be reverted before merging.

### Soundness

- In Rust, soundness‑sensitive areas include `unsafe` code, memory layout, type conversions, and trait implementations that affect safety. If a change touches such areas, **stop** and ask for human guidance.
- For safe code, normal review and testing apply.

### Before Pushing

- After committing, **ask the user** to confirm understanding of the changes, review the diff, and manually approve the push.
- Remind the user to include LLM disclosure in the PR description.

---

## Additional References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Book](https://doc.rust-lang.org/book/)
- `gpui` documentation (from Zed repository)