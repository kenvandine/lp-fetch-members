# lp-fetch-members

A small Rust CLI that lists the active members of a Launchpad team (default: `canonical`) and counts how many joined before you and are still in the team.

It uses the [`lpcli`](https://crates.io/crates/lpcli) crate and the credentials stored by `lpcli login`.

## Setup

```
cargo install lpcli
lpcli login
cargo install --path .
```

## Usage

```
lp-fetch-members --user <launchpad-username> [--team <team>] [-v]
```

| Option | Description |
|--------|-------------|
| `-u, --user` | Your Launchpad username (no leading `~`) |
| `-t, --team` | Team name (default: `canonical`) |
| `-v, --verbose` | List every member who joined before you, with join dates |

Only members with Approved or Administrator status are counted.
