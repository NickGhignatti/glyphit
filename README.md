## GlyphIt

GlyphIt is a super-fast, cross-platform Rust CLI tool for expressive Git commits, integrating emoji conventions to make 
your commit history more readable and fun. GlyphIt gives you interactive commit prompts, automatic emoji selection, 
and fully integrates as a commit hook for seamless team adoption.

### Features
- 🚀 **Blazing fast**: Written in Rust for instant startup and low resource usage
- 🎨 **Emoji-powered commits**: Add standardized emojis to your commit messages
- 📝 **Interactive CLI**: Easy prompts for commit message creation, emoji selection, scope, and more
- 🔍 **Search & list**: Find emojis by keyword, view all available conventions
- 🤝 **Conventional Commits support**: Standards-friendly workflows
- 🛠️ **Cross-platform**: Linux, macOS, Windows binaries

### Quickstart
Initialize GlyphIt hooks:
```bash
glyphit init
```

Usage:
```bash
glyphit --help
```

### Configuration
GlyphIt reads per-repo config automatically from your .git/config, including username, email, active branch, and remote URLs.