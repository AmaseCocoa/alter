# Alter
One-liner tool for switching Git identities and credentials.

## Why this tool?

Managing multiple Git accounts often requires switching `user.name`, `user.email`, GPG signing keys, and `credential.namespace`.
Doing this manually with `git config` is repetitive and doesn’t scale well—especially when you work across personal and organizational projects.

**Alter** was built to make this workflow smoother.

* Each Git identity (username, email, GPG key, credential namespace) is stored as an isolated profile
* You can switch identities with a single, consistent command
* Project-specific setups can be applied cleanly with `--local`

The tool originally started as a personal project written in another language.
It was later rewritten in Rust, not only for performance, but also because of Rust’s strong Git-related ecosystem—especially crates like **`gix_config`** from the gitoxide project.
This ecosystem made Rust a natural fit for building a fast, reliable identity-switching tool.

## Install

### Cargo
```bash
cargo install alter
```

### Arch Linux (AUR)
```bash
paru -S alter
```

## Usage

### Create a new profile
```bash
alter new
```

### Delete a profile
```bash
alter delete <slug>
```

### Switch to a profile
```bash
alter use <slug> [--local]
```

### List available profiles
```bash
alter list
```

## License
MIT