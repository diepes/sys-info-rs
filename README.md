# sys-info-rs

* Cloned from - https://github.com/FillZpp/sys-info-rs.git

[![Build Status](https://travis-ci.org/FillZpp/sys-info-rs.svg?branch=master)](https://github.com/diepes/sys-info-rs.git)

Get system information in Rust.

For now it the focus is on Linux and Mac OS and Windows.
It may also still work on Linux, Mac OS X, illumos, Solaris, FreeBSD, OpenBSD, NetBSD and Windows.
And now it can get information of kernel/cpu/memory/disk/load/hostname and so on.

[Documentation](https://github.com/diepes/sys-info-rs.git)

### Usage
Add this to `Cargo.toml`:

```
[dependencies]
sys-info = { git = "https://github.com/diepes/sys-info-rs.git" }
```

and add this to crate root:

```
extern crate sys_info;
```
