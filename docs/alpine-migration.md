# Alpine Linux Migration Plan

Tujuan: ganti base OS dari Ubuntu (glibc) ke Alpine Linux (musl) agar image OS Helios sekecil mungkin (~30–60 MB rootfs).

## Kenapa Alpine

Alpine adalah satu-satunya distro mainstream yang punya Mesa dicompile untuk musl. Semua deps tersedia:
- `mesa-egl`, `mesa-dri-gallium` (musl build)
- `libseat`, `libinput`, `libdrm`, `libgbm` (musl build)
- Rust target: `x86_64-unknown-linux-musl`

## Target stack

```
Alpine (musl base, ~5 MB)
  └── Mesa musl (EGL + OpenGL ES)
  └── libseat, libdrm, libinput, libgbm (musl)
  └── helios-comp  (Rust, x86_64-unknown-linux-musl)
  └── helios-dock  (Rust, x86_64-unknown-linux-musl)
  └── helios-bar   (Rust, x86_64-unknown-linux-musl)
```

Perbandingan ukuran rootfs:

| Base | Libc | Estimasi rootfs |
|------|------|----------------|
| Ubuntu minimal | glibc | ~200–400 MB |
| Debian slim | glibc | ~100–200 MB |
| Alpine + Mesa | musl | ~30–60 MB |

## Langkah-langkah

### Step 1 — Setup Alpine VM / container sebagai build environment

Boot Alpine di QEMU atau gunakan Docker untuk test build awal.

Install build deps:
```sh
apk add rust cargo git make gcc musl-dev pkgconf
apk add mesa-dev libseat-dev libinput-dev eudev-dev
apk add libdrm-dev wayland-dev pixman-dev
apk add libdisplay-info-dev xkeyboard-config libxkbcommon-dev
```

### Step 2 — Test build helios-comp (paling berisiko, mulai dari sini)

```sh
cd helios-comp && cargo build
```

Catatan: `libudev` di Alpine → `eudev-dev`. Kalau `libudev-sys` gagal, coba:
```sh
export PKG_CONFIG_PATH=/usr/lib/pkgconfig
```

### Step 3 — Test build helios-dock dan helios-bar

```sh
cargo build -p helios-dock -p helios-bar
```

Lebih mudah dari compositor karena deps lebih sedikit.

### Step 4 — Verifikasi binary musl

```sh
ldd ./target/debug/helios-dock   # harus "not a dynamic executable" atau hanya musl
file ./target/debug/helios-comp
```

### Step 5 — Build minimal Alpine rootfs untuk Helios

Buat minimal rootfs hanya berisi:
- musl runtime
- mesa-egl + mesa-dri-gallium
- libseat + seatd
- helios-comp, helios-dock, helios-bar binaries

Target: rootfs < 60 MB.

## Risiko yang diketahui

| Risiko | Kemungkinan | Mitigasi |
|--------|-------------|----------|
| libudev-sys tidak kenal eudev di Alpine | Sedang | Set `PKG_CONFIG_PATH` atau patch build.rs |
| libcosmic pull dep yang glibc-only | Rendah | Cek di Alpine dulu sebelum lanjut |
| Mesa version mismatch | Rendah | Alpine edge punya Mesa versi baru |
| helios-comp link fail (missing .so) | Sedang | Iterasi `apk add` deps satu per satu |

## Status

**Belum dimulai.** Mulai dari Step 1 (setup Alpine) lalu langsung Step 2 (build helios-comp).
