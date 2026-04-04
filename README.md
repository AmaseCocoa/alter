# Alter

A one-liner tool for switching Git identities and credentials with ease.

[![Crates.io](https://img.shields.io/crates/v/alter.svg)](https://crates.io/crates/alter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)

## Overview

**Alter** simplifies managing multiple Git identities by allowing you to switch between different Git profiles in a single command. Whether you're juggling personal and work projects, or managing multiple GitHub accounts, Alter handles the complexity of switching usernames, emails, GPG signing keys, and credential namespaces automatically.

### Why Alter?

Managing multiple Git accounts is cumbersome:

- Manually switching `user.name` and `user.email` across projects is error-prone
- GPG signing keys require separate configuration
- Credential helpers don't scale well across multiple accounts
- Different platforms (GitHub, GitLab, etc.) need different credentials

**Alter** solves this by:

- ✨ **Storing profiles** - Each identity is saved as an isolated, reusable profile
- ⚡ **One-command switching** - Switch identities with a single consistent command
- 🔐 **Secure credentials** - Credentials stored with OS-level security (keyring support)
- 🔑 **GPG integration** - Automatic GPG signing key configuration
- 🌐 **OAuth2 support** - Setup credentials for GitHub, GitLab, and other Git hosts
- 📱 **Project-scoped settings** - Apply configurations globally or to specific repositories

## Features

- **Profile Management**: Create, list, delete, and switch between Git identity profiles
- **Git Configuration**: Automatically configure `user.name`, `user.email`, and `commit.gpgsign`
- **Credential Management**: Store and manage credentials for multiple Git hosts
- **OAuth2 Authentication**: Setup OAuth2 credentials for GitHub, GitLab, and custom hosts
- **Keyring Integration**: Secure credential storage using the system keyring
- **Git Credential Helper**: Works as a Git credential helper for seamless authentication
- **Verbose Logging**: Debug mode for troubleshooting (`--verbose` flag)

## Installation

### Via Cargo (Recommended)

```bash
cargo install alter
```

### Via AUR (Arch Linux)

```bash
paru -S alter

# if use pre-built binary
paru -S alter-bin
```

Or with `yay`:

```bash
yay -S alter

# if use pre-built binary
yay -S alter-bin
```

### From Source

```bash
git clone https://github.com/AmaseCocoa/alter.git
cd alter
cargo install --path .
```

## Quick Start

### 1. Create Your First Profile

```bash
alter new
```

You'll be prompted to enter:

- Profile slug (unique identifier, e.g., `work`, `personal`)
- Git username
- Git email
- GPG signing key (optional)

### 2. Switch to a Profile

```bash
alter use work
```

This applies the profile globally. To apply it only to the current repository:

```bash
alter use work --local
```

### 3. Verify Current Profile

```bash
alter current
```

Shows your current active profile and authentication status.

### 4. List All Profiles

```bash
alter list
```

Displays all available profiles with their configurations.

## Usage Guide

### Profile Management

#### Create a New Profile

```bash
alter new
```

Interactive prompts guide you through creating a new Git identity profile.

#### List All Profiles

```bash
alter list
```

Shows all saved profiles with username and email information.

#### Show Current Profile

```bash
alter current
```

Displays the currently active profile and credential status.

#### Switch to a Profile

```bash
alter use <slug>
```

Switch to a specific profile globally:

```bash
alter use work
```

Switch to a profile for the current repository only:

```bash
alter use work --local
```

Reset to default Git configuration:

```bash
alter use
```

#### Delete a Profile

```bash
alter delete <slug>
```

Permanently removes a profile and its associated credentials:

```bash
alter delete personal
```

### Credential Management

#### Setup OAuth2 Credentials

```bash
alter cred setup <profile> [--host github.com]
```

Configure OAuth2 credentials for a Git host:

```bash
alter cred setup work --host github.com
```

#### List Profile Credentials

```bash
alter cred list <profile>
```

View all configured credentials for a profile:

```bash
alter cred list work
```

#### Revoke Credentials

```bash
alter cred revoke <profile> [--host github.com]
```

Remove credentials for a specific host:

```bash
alter cred revoke work --host github.com
```

#### Debug Credentials

```bash
alter cred debug <profile> [--host github.com]
```

Troubleshoot credential storage and retrieval.

### Git Credential Helper

Alter can be configured as a Git credential helper for automatic authentication.

Add to your Git configuration:

```bash
git config --global credential.helper alter
```

## Configuration

Profiles are stored in your user config directory:

- **Linux/macOS**: `~/.alter/`
- **Windows**: `%APPDATA%\.alter\`

Each profile is a TOML file containing:

```toml
[profile]
id = "550e8400-e29b-41d4-a716-446655440000"

[user]
username = "john-doe"
email = "john@example.com"
signingkey = "1234567890ABCDEF"

[credentials]
hosts = ["github.com", "gitlab.com"]
```

Credentials are securely stored in your system keyring, not in plain text files.

## Examples

### Setup for Multiple GitHub Accounts

```bash
# Create work profile
alter new
# Profile slug: work
# Username: john-work
# Email: john@work.company.com
# Signing key: (skip)

# Create personal profile
alter new
# Profile slug: personal
# Username: john-personal
# Email: john@example.com
# Signing key: (skip)

# Setup credentials for work account
alter cred setup work --host github.com

# Setup credentials for personal account
alter cred setup personal --host github.com

# Switch to work profile
alter use work

# Now git commands will use the work account
git push origin main

# Switch to personal profile
alter use personal

# Now git commands will use the personal account
git push origin main
```

### Local Repository Configuration

```bash
# Switch globally to work profile
alter use work

# But in a specific personal project, override locally
cd ~/projects/personal-project
alter use personal --local

# This project now uses the personal profile
# while other projects use the work profile
```

### Use with GPG Signing

```bash
# Create profile with GPG key
alter new
# Profile slug: secure-work
# Username: john-doe
# Email: john@company.com
# Signing key: 1234567890ABCDEF

# Switch to profile
alter use secure-work

# All commits will be signed with the specified GPG key
git commit -m "My signed commit"

# Verify signing is configured
git config user.signingkey
# Output: 1234567890ABCDEF
```

## Advanced Features

### Verbose Output

Enable detailed logging for debugging:

```bash
alter --verbose <command>
```

Example:

```bash
alter --verbose cred setup work --host github.com
```

### OAuth2 Providers

Alter supports OAuth2 authentication for multiple Git providers. When setting up credentials, you can specify custom hosts:

```bash
alter cred setup myprofile --host gitlab.company.com
```

## Troubleshooting

### Credentials Not Being Used

1. Verify the profile is active:

   ```bash
   alter current
   ```

2. Check credential configuration:

   ```bash
   alter cred list <profile>
   ```

3. Enable verbose mode for debugging:
   ```bash
   alter --verbose cred debug <profile>
   ```

### Reset to Default Configuration

To stop using Alter and reset to your default Git configuration:

```bash
alter use
```

This clears any profile-specific settings.

### Keyring Issues

If you encounter keyring-related errors:

- On Linux, ensure a keyring daemon is running (usually part of your desktop environment)
- On macOS, ensure access to the system Keychain is permitted
- On Windows, ensure Windows Credential Manager is accessible

## Architecture

**Alter** is built on:

- **Rust** for performance and safety
- **`gix-config`** from the gitoxide project for reliable Git configuration handling
- **`clap`** for command-line parsing
- **`keyring`** crate for secure credential storage
- **`reqwest`** for OAuth2 HTTP requests

## Building from Source

### Requirements

- Rust 1.70 or later
- Cargo

### Build Steps

```bash
git clone https://github.com/AmaseCocoa/alter.git
cd alter
cargo build --release
```

The compiled binary will be at `target/release/alter`.

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request to the [GitHub repository](https://github.com/AmaseCocoa/alter).

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

Copyright (c) 2025-2026 AmaseCocoa

## Support

For issues, feature requests, or questions, please visit the [GitHub Issues](https://github.com/AmaseCocoa/alter/issues) page.

---

Made with ❤️ for developers managing multiple Git identities
