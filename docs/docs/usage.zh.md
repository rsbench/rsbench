# 使用方法

本项目分为四个大模块

- INFO:
    - OS: 操作系统信息
    - CPU: CPU 信息
    - MEM: 内存信息
    - DISK: 磁盘信息
    - SWAP: 交换分区信息
    - KERN: 内核信息
    - VIRT: 虚拟化信息

    输出信息仅供参考，不作为判断性能依据
- BENCH:
    - FIB: 斐波那契数列运算性能测试
    - DISK: 磁盘顺序读写测试 (BUG 较多暂不开放)
    - MEM: 内存读写测试
    - PING: 网络延迟测试 (Cloudflare TCPing)
    - LOSS: 网络丢包测试 (Cloudflare TCPing)
    - NET: 网络测试 (仅为与 Cloudflare 测速服务器连接测试，实际多路测试请看下方 Speedtest)
    测试结果仅供参考，不作为判断性能依据
- TUNE:
    - Speedtest: 测试网络速度
- UNLOCK:
    互联网服务解锁测试，用于测试用户是否可以正常使用互联网服务，如流媒体、游戏平台等

## INFO

当仅执行 Binary 而不传入任何参数时，默认执行 `INFO` 模块

使用方法: 
```bash
rsbench -i 
```

该功能无其他参数，执行后会输出主机信息:
```
OS  : Arch Linux rolling
CPU : AMD Ryzen 7 6800H with Radeon Graphics 16 Threads @ 2333Mhz
MEM : 13 GB
DISK: 57/185 GB
DISK: 35/156 MB
SWAP: 8.00 GB
KERN: 6.12.6-zen1-1-zen
VIRT: none
```

## BENCH

该模块会全面评估主机性能，目前已经实现了 `网络测试 / CPU 性能测试 / 磁盘顺序读写测试` 等功能，更多的还在持续更新中

**PS: 该模块可能会尽力占用系统资源，请谨慎使用**

使用方法: 
```bash
rsbench -b [OPTIONS]
```

可选参数: 
- `--network`: 执行网络测试
- `--fib`: 执行 CPU FIB 运算性能测试
- `--disk`: 执行磁盘顺序读写测试
- `--mem`: 执行内存读写测试

在不带任何参数的情况下，会执行所有测试

输出: (如果只开启了部分测试项，则不会输出全部信息)
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
MEM : 1303.6 MB/s | 14787  MB/s
PING: 1.22 ms
LOSS: 0.00%
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### 网络测试

对应参数: `--network`

`PING` 是测试连接到 Cloudflare `1.1.1.1:443` 的 TCPing 延迟，单位 ms:
```
PING: 1.22 ms
```

该测试会连接到 Cloudflare Speedtest 服务器，测试下载速度和上传速度

前两行为单线程下载速度 (以🔽为标志)，左右两边分别为 *平均速度* 和 *最大速度*，单位 Mbps，显示 5 位数字:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

后两行为多线程下载速度 (以⏬为标志)，左右两边分别为 *平均速度* 和 *最大速度*，单位 Mbps，显示 5 位数字:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB 运算测试

对应参数: `--fib`

该测试会使用 CPU 运算斐波那契数列，并计算用时以及得分

基准为使用 [机械革命蛟龙 16 Pro](https://www.mechrevo.com/content/details92_4817.html) 规格进行测试，得分为 `基准 / 测试用时`，得分越高性能越好

上下分别为 *单线程得分* 和 *多线程得分*:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### 磁盘顺序读写测试

对应参数: `--disk`

该测试会使用磁盘进行顺序读写测试，并计算用时以及得分

对于 Windows，默认的测试文件存放路径为 `C:\\rsbench_disk_test`、对于其他系统，则为 `/tmp/rsbench_disk_test`

左为写入速度，右为读取速度，单位 MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE

使用方法: 
```bash
rsbench -t [OPTIONS]
```

可选参数:
- `--speedtest`: 执行 Speedtest 测试

在不带任何参数的情况下，会执行所有测试

### Speedtest

对应参数: `--speedtest`

该测试会使用 Speedtest.net / cn 进行单线程 / 多线程测试，输出详细信息

可选参数:

- `--custom-speedtest-host <CUSTOM_SPEEDTEST_HOST>`: 自定义 Speedtest 服务器地址，格式 `DOMAIN:PORT`，不需要 `http` 等协议前缀

示例输出:

```bash
Single Thread Speedtest: 
 Provider      | Avg Down | Max Down | Avg Up | Max Up 
---------------+----------+----------+--------+--------
 Speedtest.net | 34.62    | 34.22    | 39.64  | 63.67 
 China Mobile  | 87.61    | 87.02    | 46.95  | 70.23 
 China Unicom  | 96.34    | 97.35    | 46.49  | 63.04 
 China Telecom | 30.98    | 30.62    | 45.93  | 67.04 
 HK I3D        | 0.28     | 0.77     | 28.87  | 59.26 
 TW HiNet      | 84.80    | 84.50    | 52.70  | 86.57 
 JP xTom       | 0.69     | 0.71     | 14.57  | 31.89 

Multi Thread Speedtest: 
 Provider      | Avg Down | Max Down | Avg Up | Max Up 
---------------+----------+----------+--------+--------
 Speedtest.net | 77.43    | 77.14    | 49.71  | 90.23 
 China Mobile  | 91.54    | 91.58    | 47.96  | 65.48 
 China Unicom  | 100.87   | 101.75   | 45.47  | 58.95 
 China Telecom | 28.76    | 78.35    | 48.68  | 72.82 
 HK I3D        | 0.47     | 1.07     | 50.97  | 99.50 
 TW HiNet      | 92.11    | 91.96    | 65.39  | 119.48 
 JP xTom       | 25.72    | 68.65    | 56.44  | 159.33 
```

默认测速服务器:

- `Speedtest.net`: 默认测速服务器，根据 API 选择最近的测速服务器
- `China Mobile`: speedtest1.sc.chinamobile.com:8080
- `China Unicom`: 36.250.1.90:8080
- `China Telecom`: speedtest1.online.sh.cn:8080
- `HK I3D`: hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080
- `TW HiNet`: ntp1.chtm.hinet.net:8080
- `JP xTom`: speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080

有关更多信息，可以参考 [
`12101111 Blog 的此文`](https://12101111.github.io/speedtest-net/#https-www-speedtest-net-api-js-servers-engine-js) 与
测速服务器收集列表: [CN](https://github.com/spiritLHLS/speedtest.cn-CN-ID) / [Net](https://github.com/spiritLHLS/speedtest.net-CN-ID)

## UNLOCK

该模块会多线程检测用户是否可以正常使用互联网服务，如流媒体、游戏平台等

使用方法:
```bash
rsbench -u [OPTIONS]
```

可选参数:
- `--region [location]`: 指定地区，可指定多个，如 `--region cn --region us`

在默认情况下，不指定区域则为测试所有地区的互联网服务

**请注意，由于解锁测试脚本具有一定的时效性以及特殊性，可能会出现与实际不符的情况！**

示例输出:
```bash
rsbench -u

UNLOCK:
 Y/N             Service             Error
[ Y ]           myTV Super          
[ Y ]    Bilibili China Mainland    
[ Y ]      Youtube Premium (HK)     
[ Y ]       IQIYI Oversea (HK)      
[ Y ]             ViuTV             
[ Y ] Google Play Store (Hong Kong) 
[ Y ]          Steam (HKD)          
[ Y ]       Youtube CDN (HKG)       
[ Y ]           Dazn (HK)           
[ Y ]           Hulu Japan          
[ Y ]       Bahamut Anime (HK)      
[ Y ]          HBO MAX (US)         
[ N ]    Bilibili China HK/MO/TW     Not available
[ N ]         Kancolle Japan        
[ N ]           AnimeFesta          
[ N ]     Bilibili China TW Only     Not available
[ N ] Princess Connect Re:Dive Japan Not available
[ N ]             Lemino            
[ N ]              4GTV              Not available
[ N ]           HamiVideo            Not available
[ N ]          BBC iPlayer           Not available
[ N ]              Mora             
[ N ]             U-Next             Not available
[ N ]             Now E             
[ N ]            Netflix             Not available
[ N ]            Showmax             Not available
[ N ]            Sling TV            Not available
Tested 27 projects took 3.24 seconds, 12 services unlocked, 15 services locked.
```

指定地区: 
```bash
rsbench -u --region cn --region us

UNLOCK:
 Y/N             Service             Error
[ Y ]    Bilibili China Mainland    
[ Y ]       IQIYI Oversea (HK)      
[ N ]            Sling TV            Not available
Tested 3 projects took 2.32 seconds, 2 services unlocked, 1 services locked.
```

目前已经分类的地区:
```
hk, mo, tw, jp, cn, asia, euro, afr, uk, us, global
```

`global` 为在全球范围内提供服务的平台，如 Netflix (极小部分地区不支持也算)

## 上传至 Pastebin

在默认情况下，测试结束后会自动上传测试结果至 Pastebin，你可以通过以下参数来控制是否上传:

- `--no-upload`: 禁用上传

默认上传至 <https://pastebin.highp.ing>，项目地址为: <https://github.com/rsbench/pastebin>

由于部署于 Cloudflare Workers，数据库采用 D1，拥有一些限制，所以只好打开鉴权

在官方 Github Actions 编译中，我们会内置鉴权信息，所以在安装时无需担心鉴权问题

如果是自行编译，你需要自行配置鉴权信息，具体请参考本文档 (还没写)

## 通用参数

- `--help`: 显示帮助信息
- `--version`: 显示版本信息
- `--no-color`: 禁用颜色输出
- `--no-cls`: 禁用清屏，默认在程序执行前清屏
- `--no-logo`: 不输出 Ascii Art Logo
- `--no-usage`: 不提交使用次数统计
- `--no-upload`: 禁用上传测试结果至 Pastebin

在默认情况下，你可以将四个模块的功能合并在一起使用，比如:
```bash
rsbench -ibtu --network --region cn
```

或者直接全量运行测试:
```bash
rsbench -ibtu
```
