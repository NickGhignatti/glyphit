# 🔧 Commands

GlyphIt provides a small but powerful set of subcommands.  
All commands are implemented in the `src/functions/` module.

---

## `glyphit add`

Stages files for commit with emoji tagging.

```bash
glyphit add [files...]
```

Example:
```bash
glyphit add src/main.rs
```

Internally, it calls:
```rust
git add <files>
```
and prepares commit metadata with the correct emoji.

---

## `glyphit commit`

Creates a Git commit with the standardized emoji prefix.

```bash
glyphit commit
```

Example:
```bash
glyphit commit
# Then it will start an interactive prompt
# => 🐛 fix: memory leak
```

GlyphIt automatically determines the emoji by scanning the message or type keyword.

---

## `glyphit push`

Pushes your changes to the current branch.

```bash
glyphit push
```

Equivalent to:
```bash
git push origin HEAD
```