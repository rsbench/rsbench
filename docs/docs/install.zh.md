# 安装

我们正在努力支持多平台的一键安装...

即使如此，安装方式也很简单

*PS: 下方的所有方式均以 Linux 为例，即使它们也可用于其他设备*

## Cargo Install Binary 安装

如果你的执行设备拥有了 Cargo 以及编译环境，那么以这种方式安装是**最简单、最能保证可用性**的

```bash
> cargo install rsbench
```

安装成功后，执行 `rsbench` 命令，查看是否输出 `RSBench` Ascii-art 来检验是否安装成功即可

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

## Binary 安装

此方法安装方法同样也很简单，首先请前往本项目 [Release 界面](https://github.com/rsbench/rsbench/releases/tag/latest) 寻找符合执行设备系统 / 架构的 Binary (即为不带任何后缀的文件)，复制其链接

然后使用各种方法将 Binary 下载 / 上传至执行设备即可 (此处以 Wget CLI 下载为例):
```bash
> wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx # 务必替换下载链接
```

然后为其附上可执行权限: 
```bash
> chmod +x rsbench
```

如果具有该设备的**特权**，并且希望将其加入 PATH 环境变量中，可以将其移动至 `/usr/bin/rsbench`: (可选)
```bash
> mv ./rsbench /usr/bin/rsbench # 务必确保拥有设备特权
```

随后执行 `./rsbench` 命令，查看是否输出 `RSBench` Ascii-art 来检验是否安装成功即可

*PS: 如果你已经将其放入 `/usr/bin/`，则不需要附上本地路径，直接运行 `rsbench` 即可*

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

## Debian 系列发行版 Deb 包安装

该方法仅适用于 Debian 系列的 Linux 发行版，例如 Ubuntu 及其各种衍生版，且必须具备系统**特权**

请前往本项目 [Release 界面](https://github.com/rsbench/rsbench/releases/tag/latest) 寻找符合执行设备系统 / 架构的 Deb 包 (以 .deb 为后缀的文件)

然后使用各种方法将 Binary 下载 / 上传至执行设备即可 (此处以 Wget CLI 下载为例):
```bash
> wget -O rsbench https://github.com/rsbench/rsbench/releases/download/latest/rsbench_xxxx.deb # 务必替换下载链接
```

其次，用系统**特权**用户安装本 Deb 包:
```bash
> dpkg -i ./rsbench_xxx.deb
```

安装成功后执行 `rsbench` 命令，查看是否输出 `RSBench` Ascii-art 来检验是否安装成功即可

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