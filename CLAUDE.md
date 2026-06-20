# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Helios OS — a custom Linux desktop environment built in Rust. GPL-3.0.

Workspace members (root `Cargo.toml`): `helios-dock`, `helios-bar`, `hpkg`.  
`helios-comp/` is a separate Rust workspace (fork of cosmic-comp) — not part of the root workspace.

## Build & Run

```bash
cargo build                        # all workspace members
cargo build -p helios-dock         # single crate
cargo build --release              # production build
```

`helios-dock` and `helios-bar` are Wayland layer-shell clients — they require a running compositor. Do not use `cargo run` directly.

For `helios-comp` (compositor):
```bash
cargo build -p helios-comp         # wrong — helios-comp is a separate workspace
cd helios-comp && cargo build      # correct
cd helios-comp && make DEBUG=0     # release build via Makefile
```

### Running the full stack

```bash
# 1. Start compositor (from tty1 or via seatd)
XDG_RUNTIME_DIR=/run/user/1000 XDG_SEAT=seat0 XDG_VTNR=1 LIBSEAT_BACKEND=seatd \
  ./helios-comp/target/debug/cosmic-comp &

# 2. Start bar and dock
WAYLAND_DISPLAY=wayland-1 XDG_RUNTIME_DIR=/run/user/1000 ./target/debug/helios-bar &
WAYLAND_DISPLAY=wayland-1 XDG_RUNTIME_DIR=/run/user/1000 ./target/debug/helios-dock &
```

## Linker (critical)

`.cargo/config.toml` overrides the linker for `x86_64-unknown-linux-gnu` — rustup's bundled `lld` crashes on this machine. Uses `gcc` + `ld.gold` instead. Do not remove or change this config.

## Format & Lint

```bash
cargo fmt --all                                        # format all crates
cargo clippy --all-features -- -D warnings             # lint (treats warnings as errors)
```

CI (`helios-comp/.github/workflows/ci.yml`) runs format, clippy, feature checks, and tests against `stable` toolchain. Root workspace uses `rustup` channel from `helios-comp/rust-toolchain.toml` (`1.90`).

## Style

- Rust edition 2024 throughout.
- `rustfmt.toml` in `helios-comp/` sets `style_edition = "2024"`.
- Clippy with `-D warnings` — fix all warnings before committing.

## Key Dependencies

- `libcosmic` (git, `wayland` + `tokio` features) — GUI framework for `helios-dock` and `helios-bar`. Uses `zwlr-layer-shell-v1` to anchor surfaces to screen edges.
- `tokio` (full features) — async runtime.
- `serde` + `toml` — config serialization.

## Remote

```
git@github.com:gilang-as/helios.git
```
