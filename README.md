# 🎨 GlyphIt

> **Emoji-powered Git commits made simple**

GlyphIt is a blazingly fast, Rust-based CLI tool that brings the power of conventional commits with emojis to your Git workflow. Say goodbye to boring commit messages and hello to expressive, standardized commits that communicate intent at a glance.

## ✨ Features

- **🎯 Interactive Emoji Selection** - Choose from a curated list of conventional commit emojis
- **⚡ Lightning Fast** - Built with Rust for maximum performance
- **🔒 Type-Safe** - Leverages Rust's type system for reliability
- **📦 Zero Dependencies** - Self-contained binaries for all major platforms
- **🎨 Gitmoji Compatible** - Follows the popular gitmoji convention
- **🔄 Full Git Integration** - Seamlessly works with your existing Git workflow

## 🚀 Quick Start

```bash
# Add files to staging
glyphit add file1.txt file2.txt

# Create an emoji-powered commit
glyphit commit

# Push to remote
glyphit push
```

## 📥 Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/yourusername/glyphit/releases):

#### Linux (x86_64)
```bash
# Download and extract
curl -LO https://github.com/yourusername/glyphit/releases/latest/download/glyphit-Linux-musl-x86_64.tar.gz
tar -xzf glyphit-Linux-musl-x86_64.tar.gz

# Move to PATH
sudo mv glyphit /usr/local/bin/
```

#### Linux (ARM64)
```bash
# Download and extract
curl -LO https://github.com/yourusername/glyphit/releases/latest/download/glyphit-Linux-musl-arm64.tar.gz
tar -xzf glyphit-Linux-musl-arm64.tar.gz

# Move to PATH
sudo mv glyphit /usr/local/bin/
```

#### macOS (x86_64)
```bash
# Download and extract
curl -LO https://github.com/yourusername/glyphit/releases/latest/download/glyphit-macOS-x86_64.tar.gz
tar -xzf glyphit-macOS-x86_64.tar.gz

# Move to PATH
sudo mv glyphit /usr/local/bin/
```

#### Windows (x86_64)
Download `glyphit-Windows-msvc-x86_64.zip` from the releases page, extract it, and add the directory to your PATH.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/glyphit.git
cd glyphit

# Build with cargo
cargo build --release

# The binary will be in target/release/glyphit
```

## 📖 Usage

### Adding Files

Add files to the staging area just like `git add`:

```bash
glyphit add src/main.rs
glyphit add file1.txt file2.txt file3.txt
```

### Creating Commits

Launch the interactive commit creator:

```bash
glyphit commit
```

You'll be prompted to:
1. **Select an emoji** from the conventional commit types
2. **Enter a commit message** describing your changes
3. **Add breaking changes** (optional)

**Available Emoji Types:**

| Emoji | Code | Description |
|-------|------|-------------|
| 🎨 | `:art:` | Improve structure/format of the code |
| ⚡️ | `:zap:` | Improve performance |
| 🔥 | `:fire:` | Remove code or files |
| 🐛 | `:bug:` | Fix a bug |
| ✨ | `:sparkles:` | Introduce new features |

### Pushing Changes

Push your commits to the remote repository:

```bash
glyphit push
```

Supports both HTTPS and SSH authentication methods.

## 🎯 Complete Workflow Example

```bash
# Make your changes
echo "new feature" > feature.txt

# Stage the changes
glyphit add feature.txt

# Create an emoji commit
glyphit commit
# Select: ✨ :sparkles: Introduce new features
# Message: add new user authentication feature
# Breaking: (leave empty)

# Push to remote
glyphit push
```

## 🏗️ Architecture

GlyphIt is built with modern software engineering principles:

- **Domain-Driven Design** - Clear separation of concerns with domain types
- **Test-Driven Development** - Comprehensive test coverage for reliability
- **Functional Core** - Pure functions with minimal side effects
- **Type Safety** - Leverages Rust's strong type system

### Project Structure

```
glyphit/
├── src/
│   ├── main.rs           # Entry point
│   ├── functions/        # Core Git operations
│   │   ├── add.rs        # File staging
│   │   ├── commit.rs     # Commit creation
│   │   └── push.rs       # Remote pushing
│   └── types/            # Domain types
│       ├── commands.rs   # CLI command definitions
│       └── repository.rs # Repository utilities
├── docs/                 # Documentation
└── .github/workflows/    # CI/CD pipelines
```

## 🔧 Configuration

GlyphIt uses your existing Git configuration:

```bash
# Set your Git identity (if not already set)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Run tests** (`cargo test`)
5. **Commit with GlyphIt** (`glyphit commit`)
6. **Push to your fork** (`glyphit push`)
7. **Open a Pull Request**

### Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/glyphit.git
cd glyphit

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with debug output
cargo run -- add file.txt
cargo run -- commit
```

## 📋 Requirements

- **Rust Edition**: 2024
- **Git**: 2.0 or higher
- **Platforms**: Linux (x86_64, ARM64), macOS (x86_64), Windows (x86_64)

## 🐛 Troubleshooting

### Authentication Issues

**HTTPS Authentication:**
```bash
# Configure credential helper
git config --global credential.helper store
```

**SSH Authentication:**
```bash
# Ensure SSH agent is running with your key
ssh-add ~/.ssh/id_rsa
```

### Repository Not Found

Make sure you're in a Git repository:
```bash
git status
```

## 📄 License

This project is licensed under the GLPv3 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [gitmoji](https://gitmoji.dev/) - An emoji guide for commit messages
- Built with [git2-rs](https://github.com/rust-lang/git2-rs) - Rust bindings to libgit2
- Powered by [clap](https://github.com/clap-rs/clap) - Command line argument parser

## 🌟 Star History

If you find GlyphIt useful, please consider giving it a star! ⭐

---