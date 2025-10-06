# 🧩 Architecture

GlyphIt’s architecture is modular and designed around **command execution** and **repository handling**.

---

## 🗂️ Project Structure

```
src/
├── main.rs              # Entry point
├── functions/           # CLI command implementations
│   ├── add.rs
│   ├── commit.rs
│   ├── push.rs
│   └── mod.rs
└── types/               # Core types and data structures
    ├── commands.rs
    ├── repository.rs
    └── mod.rs
```

---

## ⚙️ Core Modules

### `main.rs`
- Parses CLI arguments.
- Delegates execution to command functions in `functions/mod.rs`.

### `functions/`
Contains the implementation of the main Git commands:
- `add.rs` → handles staging files.
- `commit.rs` → creates emoji-standardized commits.
- `push.rs` → handles pushing to remote.

### `types/`
Defines data models used across the codebase:
- `commands.rs` → defines enums/structs for command types.
- `repository.rs` → manages local Git repo metadata.

---

## 🧱 Design Philosophy

- Keep logic modular (per command file)
- Focus on simplicity, not abstraction
- Avoid unnecessary dependencies
