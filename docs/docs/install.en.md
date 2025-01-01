# Installation

We are striving to support one-click installation across multiple platforms...

Even so, the installation methods are quite simple.

*PS: The following instructions are given for Linux as an example, although they can also be applied to other operating systems.*

## Cargo Install Binary

If your execution device has Cargo and a compilation environment, then this is the **simplest and most reliable** way to install.

```bash
> cargo install rsbench
```

After successful installation, run the `rsbench` command and check if the output displays the `RSBench` Ascii-art to verify the installation:

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

This method is also straightforward. First, visit the [Releases page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project to find the Binary that matches your system/architecture (the file without any extension), and copy its link.

Then use various methods to download/upload the Binary to your execution device (here we use Wget CLI as an example):

```bash
> wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx # Make sure to replace with the actual download link
```

Grant it executable permissions:
```bash
> chmod +x rsbench
```

If you have **privileges** on the device and wish to add it to the PATH environment variable, you can move it to `/usr/bin/rsbench`: (optional)
```bash
> mv ./rsbench /usr/bin/rsbench # Ensure you have the necessary privileges
```

Then execute the `./rsbench` command and check if the `RSBench` Ascii-art is displayed to verify the installation:

*PS: If you've moved it to `/usr/bin/`, you don't need to prefix it with the local path; just run `rsbench`.*

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

## Debian-based Distributions Deb Package Installation

This method is only suitable for Debian-based Linux distributions like Ubuntu and its derivatives, and you must have system **privileges**.

Visit the [Releases page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project to find the Deb package matching your system/architecture (files with the `.deb` extension).

Download the Deb package to your execution device using various methods (here we use Wget CLI as an example):
```bash
> wget -O rsbench.deb https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx.deb # Replace with the actual download link
```

Next, install the Deb package using a **privileged** user:
```bash
> sudo dpkg -i ./rsbench.deb
```

After successful installation, run the `rsbench` command and check if the `RSBench` Ascii-art is displayed to verify the installation:

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

This method is only applicable for MacOS systems.

Visit the [Releases page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project to find the Binary that matches your device architecture.

The naming format will be `rsbench-macos-[amd64 | arm64]`. After downloading, you can execute it directly.

After installation, run the `./rsbench` command and check if the `RSBench` Ascii-art is displayed to verify the installation:

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