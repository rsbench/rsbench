### Installation

We are actively working to support one-click installation across multiple platforms.

Even so, the installation process is quite straightforward. The instructions below are provided with Linux in mind, but they can be adapted for other operating systems as well.

#### Cargo Install Binary

If your execution environment already has Cargo and a build toolchain installed, this method is the simplest and most reliable way to install RSBench.

```bash
> cargo install rsbench
```

After installation, you can verify its success by running `rsbench` and checking if it outputs the `RSBench` ASCII art:

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

#### Binary Installation

This installation method is also straightforward. First, visit the [Releases page](https://github.com/rsbench/rsbench/releases/tag/latest) of this project to find a binary that matches your system's architecture and operating system (look for the file without any extension), then copy the download link.

You can use various methods to download or upload the binary to your execution device. Here we use Wget CLI for downloading as an example:

```bash
wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx # Make sure to replace with the actual download link
```

Next, add executable permissions to the binary:

```bash
> chmod +x rsbench
```

If you have administrative privileges on the device and wish to add it to the PATH environment variable, you can move it to `/usr/bin/rsbench`: (optional)

```bash
> mv ./rsbench /usr/bin/rsbench # Ensure you have administrative privileges on the device
```

Then run the `./rsbench` command to check if it outputs the `RSBench` ASCII art, confirming a successful installation:

*Note: If you moved it to `/usr/bin/`, you do not need to prefix the command with `./`; simply run `rsbench`.*

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

#### Debian Package Installation

This method applies only to Debian-based Linux distributions such as Ubuntu and its derivatives, and it requires administrative privileges.

Visit the [Releases page](https://github.com/rsbench/rsbench/releases/tag/latest) to find a `.deb` package suitable for your system's architecture.

Download the .deb package using Wget CLI as an example:

```bash
> wget -O rsbench.deb https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx.deb # Replace with the actual download link
```

Then install the .deb package using administrative privileges:

```bash
> dpkg -i ./rsbench.deb
```

Finally, verify the installation by running `rsbench` and checking for the `RSBench` ASCII art output:

```bash
> rsbench 
  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

......