# Helios OS

A custom Linux desktop environment built in Rust. Targets full Wayland with no X11 dependency.

**License:** GPL-3.0

---

## Components

| Crate | Description |
|-------|-------------|
| `helios-comp` | Wayland compositor — fork of [cosmic-comp](https://github.com/pop-os/cosmic-comp) (Smithay-based) |
| `helios-dock` | Application dock — layer-shell surface anchored to the bottom edge |
| `helios-bar` | Status bar — layer-shell surface anchored to the top edge |
| `hpkg` | Package manager (WIP) |

`helios-comp` is a standalone Rust workspace under `helios-comp/`. The other crates share the root workspace.

---

## Requirements

- Linux with `seatd` or `logind`
- `virtio-gpu` or real GPU with DRM/KMS support
- Rust 1.90+ (see `helios-comp/rust-toolchain.toml`)

System dependencies:
```
libudev-dev libseat-dev libxkbcommon-dev libinput-dev
libgbm-dev libdrm-dev libwayland-dev libpixman-1-dev
libdisplay-info-dev libegl-mesa0 libgl1-mesa-dri
```

---

## Build

```bash
# Root workspace (dock, bar, hpkg)
cargo build

# Compositor (separate workspace)
cd helios-comp && cargo build
```

---

## Run

```bash
# 1. Start the compositor (requires seat access — run from TTY or via seatd)
XDG_RUNTIME_DIR=/run/user/1000 XDG_SEAT=seat0 XDG_VTNR=1 \
LIBSEAT_BACKEND=seatd \
  ./helios-comp/target/debug/cosmic-comp &

# 2. Start bar and dock
WAYLAND_DISPLAY=wayland-1 XDG_RUNTIME_DIR=/run/user/1000 \
  ./target/debug/helios-bar &

WAYLAND_DISPLAY=wayland-1 XDG_RUNTIME_DIR=/run/user/1000 \
  ./target/debug/helios-dock &
```

---

## Architecture

```
helios-comp  (Wayland compositor — KMS/DRM, libinput, seatd)
    │
    └── wayland socket (wayland-1)
          ├── helios-bar   (zwlr-layer-shell, TOP anchor, full width)
          └── helios-dock  (zwlr-layer-shell, BOTTOM anchor, centered pill)
```

The compositor handles input, rendering, and window management. Bar and dock use `zwlr-layer-shell-v1` to reserve screen edges without overlapping application windows.
