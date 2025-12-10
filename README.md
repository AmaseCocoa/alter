# Alter
Switch git username and email and credential in one-line.

## Install
```bash
cargo install alter
```

If using Arch Linux, can install from AUR:

```bash
paru -S alter
```

## Usage
Adding profile:
```bash
alter new
```

Delete profile:
```bash
alter delete [slug]
```

Switch profile:
```bash
alter use [slug] <--local>
```

List avaliable profiles:
```bash
alter list
```

## License
MIT
