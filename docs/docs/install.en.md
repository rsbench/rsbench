# Installation

We are working hard to support one-click installation on multiple platforms...

Even so, the installation methods are very simple.

*PS: All methods below use Linux as an example, even though they can also be used on other devices.*

## Cargo Install Binary Installation

If your execution device has Cargo and a compilation environment, installing in this way is the **simplest and most
reliable way to ensure availability**.

```bash
> cargo install rsbench
```

After successful installation, execute the `rsbench` command to check if the `RSBench` Ascii-art is output to verify
successful installation.

```bash
> rsbench
  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

......
```

## Binary Installation

This installation method is also very simple. First, go to
the [Release page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project and find the Binary that
matches the operating system/architecture of your execution device (i.e., the file without any suffix). Copy its link.

Then, use various methods to download/upload the Binary to the execution device (here using Wget CLI download as an
example):
```bash
> wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx # Be sure to replace the download link
```

Then, add execute permissions to it:
```bash
> chmod +x rsbench
```

If you have **privileges** on the device and want to add it to the PATH environment variable, you can move it to
`/usr/bin/rsbench`: (optional)
```bash
> mv ./rsbench /usr/bin/rsbench # Be sure you have device privileges
```

Then execute the `./rsbench` command to check if the `RSBench` Ascii-art is output to verify successful installation.

*PS: If you have already placed it in `/usr/bin/`, you do not need to include the local path; just run `rsbench`.*

```bash
> ./rsbench
  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

......
```

## Debian Series Distribution Deb Package Installation

This method is only applicable to Debian-based Linux distributions, such as Ubuntu and its various derivatives, and
requires system **privileges**.

Please go to the [Release page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project and find the Deb
package that matches the operating system/architecture of your execution device (files with the `.deb` suffix).

Then, use various methods to download/upload the Binary to the execution device (here using Wget CLI download as an
example):
```bash
> wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx.deb # Be sure to replace the download link
```

Next, install this Deb package with a system **privileged** user:
```bash
> dpkg -i ./rsbench_xxx.deb
```

After successful installation, execute the `rsbench` command to check if the `RSBench` Ascii-art is output to verify
successful installation.

```bash
> rsbench
  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

......
```

## MacOS Installation

This method is only applicable to MacOS systems.

Please go to the [Release page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project and find the
Binary that matches the architecture of your execution device (i.e., the file without any suffix).

The naming format is `rsbench-macos-[amd64 | arm64]`. Execute it after downloading.

After successful installation, execute the `./rsbench` command to check if the `RSBench` Ascii-art is output to verify
successful installation.

```bash
> ./rsbench
  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

......
```