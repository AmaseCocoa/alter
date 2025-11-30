# Alter
An "Alternative" account switcher. written in Rust

## Install
```bash
cargo install alter
```

If using Arch Linux, can install from AUR:

```
paru -S alter
```

## Usage
Adding or removing accounts is not currently supported. You can add or remove accounts by manually modifying the configuration file.
```toml
inUse = "" # USED FOR STATE MANAGEMENT, DO NOT EDIT IT!

[[accounts]]
type = "linked" # Field type, you want use alias, set type to "linked"
id = "foobar" # Unique Identifier, recommend uuid.
slug = "linked"
host = "https://example.com" # The server to which the account belongs
linkTo = "hoge" # ID of the alias you want to link
withCred = true # If true, At the same time, the authentication settings will also be changed.

[[accounts]]
type = "details"
id = "barfoo"
slug = "example"
host = "https://git.example.com"
username = "user" # username for commit, etc
email = "user@example.com" # email address for commit, etc
withCred = false
signingkey = "" # Key used for signing

[[aliases]]
id = "hoge"
username = ""
email = ""
signingkey = ""

```

Switch User:
```bash
alter
```

## License
MIT
