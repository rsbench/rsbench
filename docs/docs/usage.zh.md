# 使用方法

本项目分为四个大模块

- INFO:
    输出主机 `系统环境 / CPU / 内存 / 硬盘 / SWAP / 内核 / 虚拟化技术` 等信息

    输出信息仅供参考，不作为判断性能依据
- BENCH:
    执行基准测试，包括 `网络测试 / CPU 性能测试 / 内存性能测试 / 硬盘性能测试` 等 (目前部分功能未完成)

    测试结果仅供参考，不作为判断性能依据
- TUNE:
     - IPCheck: 检测 IP 的详细信息
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

在不带任何参数的情况下，会执行所有测试

输出: (如果只开启了部分测试项，则不会输出全部信息)
```
PING: 1.22 ms
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
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
- `--ip`: 执行网络测试

### IPCheck

对应参数: `--ip`

该测试会检测用户的 IP 地址，并输出详细信息

该测试会使用多个 IP 提供服务商进行测试，如果本地多 IP 或有对应的分流规则，将会出现多个 IP 情况，这是正常的

示例输出:
```bash
IP  :
 Provider  |                    IP                    |             Region             | Risk  | Org
Ipinfo.io                 141.**.**.220                      TW - Taiwan - Neihu          N/A    AS38136 Akari Networks
Ipinfo.io            2409:****:****:9c60::67e              CN - Shanghai - Shanghai       N/A    AS9808 China Mobile Communications Group Co., Ltd.
```

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

## 通用参数

- `--help`: 显示帮助信息
- `--version`: 显示版本信息
- `--no-color`: 禁用颜色输出
- `--no-cls`: 禁用清屏，默认在程序执行前清屏
- `--no-logo`: 不输出 Ascii Art Logo

在默认情况下，你可以将四个模块的功能合并在一起使用，比如:
```bash
rsbench -ibtu --network --region cn
```

或者直接全量运行测试:
```bash
rsbench -ibtu
```
