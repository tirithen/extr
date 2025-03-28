<div align="center">
  <pre>
  ███████╗██╗  ██╗████████╗██████╗ 
  ██╔════╝╚██╗██╔╝╚══██╔══╝██╔══██╗
  █████╗   ╚███╔╝    ██║   ██████╔╝
  ██╔══╝   ██╔██╗    ██║   ██╔══██╗
  ███████╗██╔╝ ██╗   ██║   ██║  ██║
  ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
     Effortless File Extraction
  </pre>
  
  <h1>extr</h1>
  <h3>The Archive Extractor That Just Works™</h3>

  [![Crates.io](https://img.shields.io/crates/v/extr?color=blue&style=for-the-badge)](https://crates.io/crates/extr)
  [![License: GPLv3](https://img.shields.io/crates/l/extr)](https://opensource.org/license/gpl-3-0)
</div>

---

### Extract archives without the headache

Instead of this 😖

```
$ tar xzf project.tar.gz -C ./output
```

Just do this 😎

```
$ extr project.tar.gz
```

## Features That Make You Smile 😊

- 🎯 **Zero Hassle** - Just `extr filename` no flags needed
- 🔍 **Smart Detection** - Handles 30+ formats automatically
- 🛡️ **Safe & Secure** - Uses only verified system tools
- 🌈 **Terminal Magic** - Interactive with underlying binary
- 🤖 **Self-Healing** - `extr --health` checks your setup

## Installation ➡️ Usage ➡️ Profit!

```
$ cargo install extr
```

## Usage Examples 🚀

Basic extraction (where did the complexity go?)

```
$ extr archive.zip
```

Bulk extract files 

```
$ extr ~/Downloads/the-file.zip other-file.tar.gz
```

Specify output directory

```
$ extr backup.tar.gz -o ~/restored_files
```

See what's supported

```
$ extr --health
```

## Why extr? Let's Compare 🤼

| Traditional Way                 | **extr** Way                   |
|---------------------------------|--------------------------------|
| `$ unrar x -y compressed.rar`   | `$ extr compressed.rar`        |
| `$ 7z x -o./output file.7z`     | `$ extr file.7z -o ./output`   |
| `$ unzip -q -d dest file.zip`   | `$ extr file.zip`              |

## Health Check - Know Your System 🩺

```
$ extr --health

Archive format support health check

To support a format at least one of the compatible
binaries must be installed on the system.

Format    Available Binaries
――――――――――――――――――――――――――――――――――
zip       ✓ unzip, ✓ 7z, ✘ jar
tar.gz    ✓ tar, ✓ bsdtar
7z        ✓ 7z, ✓ unar
rar       ✘ unrar, ✓ 7z

✓: installed, ✘: missing
```

## FAQ 🙋

**Q:** Is this safe?  
**A:** Absolutely! extr only uses your existing system tools - no shady binaries!

**Q:** What formats are supported?  
**A:** All of them! (Okay, 30+ including zip, tar, 7z, rar, iso, deb, rpm...)

## Contribute to the Extraction Revolution! 🫶

Found a bug? Missing your favorite format? Let's make extraction awesome together!

1. Clone the repo

```
$ git clone https://github.com/tirithen/extr.git
```

2. Build with love

```
$ cargo build --release
```

3. Submit a PR!

---

Made with ❤️ and Rust - For everyone who ever typed `tar --help` and cried
