# Usage

This project is divided into four main modules:

- INFO:
  Outputs host information such as
  `System Environment / CPU / Memory / Disk / SWAP / Kernel / Virtualization Technology`.

  The output information is for reference only and should not be used as a basis for judging performance.
- BENCH:
  Performs benchmark tests, including
  `Network Test / CPU Performance Test / Memory Performance Test / Disk Performance Test`, etc. (some features are
  currently incomplete).

  Test results are for reference only and should not be used as a basis for judging performance.
- TUNE:
  - IPCheck: Detects detailed information about an IP address.
- UNLOCK:
  Internet service unblock test, used to test whether users can normally use internet services, such as streaming media,
  game platforms, etc.

## INFO

When only the Binary is executed without passing any parameters, the `INFO` module is executed by default.

Usage:
```bash
rsbench -i
```

This function has no other parameters. After execution, it will output host information:
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

This module comprehensively evaluates host performance. It currently implements
`Network Test / CPU Performance Test / Disk Sequential Read/Write Test`, and more are being continuously updated.

**PS: This module may attempt to occupy system resources as much as possible, please use it with caution.**

Usage:
```bash
rsbench -b [OPTIONS]
```

Optional parameters:

- `--network`: Execute network test
- `--fib`: Execute CPU FIB calculation performance test
- `--disk`: Execute disk sequential read/write test

Without any parameters, all tests will be executed.

Output: (If only some test items are enabled, not all information will be output)
```
PING: 1.22 ms
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Network Test

Corresponding parameter: `--network`

`PING` tests the TCPing latency to Cloudflare `1.1.1.1:443`, in ms:
```
PING: 1.22 ms
```

This test connects to Cloudflare Speedtest servers to test download and upload speeds.

The first two lines are single-threaded download speed (marked with 🔽), with *average speed* and *maximum speed* on the
left and right, respectively, in Mbps, displaying 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The last two lines are multi-threaded download speed (marked with ⏬), with *average speed* and *maximum speed* on the
left and right, respectively, in Mbps, displaying 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Calculation Test

Corresponding parameter: `--fib`

This test uses the CPU to calculate the Fibonacci sequence and calculates the time taken and score.

The benchmark is tested using the specifications
of [Mechanical Revolution Jiaolong 16 Pro](https://www.mechrevo.com/content/details92_4817.html). The score is
`Benchmark / Test Time`, and a higher score indicates better performance.

The top and bottom are the *single-threaded score* and *multi-threaded score*, respectively:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read/Write Test

Corresponding parameter: `--disk`

This test performs sequential read and write tests on the disk and calculates the time taken and score.

For Windows, the default test file storage path is `C:\\rsbench_disk_test`. For other systems, it is
`/tmp/rsbench_disk_test`.

Left is the write speed, and right is the read speed, in MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE

Usage:
```bash
rsbench -t [OPTIONS]
```

Optional parameters:

- `--ip`: Execute IPCheck test
- `--speedtest`: Execute Speedtest

Without any parameters, all tests will be executed.

### IPCheck

Corresponding parameter: `--ip`

This test detects the user's IP address and outputs detailed information.

This test uses multiple IP service providers for testing. If there are multiple local IPs or corresponding diversion
rules, multiple IPs will appear, which is normal.

Example output:
```bash
IP  :
 Provider    | IP                        | Region                        | Risk | Org
-------------+---------------------------+-------------------------------+------+---------------------------------------------------
 Ipinfo.io   | 68.233.xxx.xx             | IN - Telangana - Srīrāmnagar  | N/A  | AS31898 Oracle Corporation
 Ipip.net    | 68.233.xxx.xx             | N/A                           | N/A  | 印度 - 特伦甘纳邦 - 海得拉巴oracle.com
 Ip.sb       | 68.233.xxx.xx             | India - Telangana - Hyderabad | N/A  | AS31898 - Oracle Cloud - ORACLE-BMC-31898
 Myip.la     | 68.233.xxx.xx             | 印度 - 特伦甘纳邦 - 海得拉巴  | N/A  | N/A
 Ipquery.io  | 68.233.xxx.xx             | India - Telangana - Hyderabad | 0    | AS31898 - Oracle Corporation - Oracle Corporation
 Vore.top    | 68.233.xxx.xx             | N/A                           | N/A  | 印度特伦加纳海得拉巴 - 甲骨文公司
 Ipcheck.ing | 68.233.xxx.xx             | 印度 - 特伦甘纳邦 - 海得拉巴  | 66   | AS31898 Oracle Corporation
 PcOnline    | 68.233.xxx.xx             | N/A                           | N/A  | N/A
 Cloudflare  | 2603:c024:8000:xxxx::xxxx | N/A                           | N/A  | N/A
 Ipinfo.io   | 2603:c024:8000:xxxx::xxxx | IN - Telangana - Srīrāmnagar  | N/A  | AS31898 Oracle Corporation
 Ip.sb       | 2603:c024:8000:xxxx::xxxx | India - Telangana - Hyderabad | N/A  | AS31898 - Oracle Cloud - ORACLE-BMC-31898
 Myip.la     | 2603:c024:8000:xxxx::xxxx | 印度 - 印度 -                 | N/A  | N/A
 Ipquery.io  | 2603:c024:8000:xxxx::xxxx | India - Telangana - Hyderabad | 0    | AS31898 - Oracle Corporation - Oracle Corporation
 Ipcheck.ing | 2603:c024:8000:xxxx::xxxx | 印度 - 特伦甘纳邦 - 海得拉巴  | 66   | AS31898 Oracle Corporation
Tested 14 projects took 5.84 seconds
```

### Speedtest

Corresponding parameter: `--speedtest`

This test uses Speedtest.net / cn to perform single-threaded / multi-threaded tests and outputs detailed information.

Optional parameter:

- `--custom-speedtest-host <CUSTOM_SPEEDTEST_HOST>`: Customize the Speedtest server address, in the format
  `DOMAIN:PORT`, without the `http` protocol prefix.

Example output:

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

Default speed test servers:

- `Speedtest.net`: Default speed test server, selects the nearest speed test server based on the API.
- `China Mobile`: speedtest1.sc.chinamobile.com:8080
- `China Unicom`: 36.250.1.90:8080
- `China Telecom`: speedtest1.online.sh.cn:8080
- `HK I3D`: hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080
- `TW HiNet`: ntp1.chtm.hinet.net:8080
- `JP xTom`: speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080

For more information, you can refer
to [this article on the 12101111 Blog](https://12101111.github.io/speedtest-net/#https-www-speedtest-net-api-js-servers-engine-js)
and the collection list of speed test
servers: [CN](https://github.com/spiritLHLS/speedtest.cn-CN-ID) / [Net](https://github.com/spiritLHLS/speedtest.net-CN-ID)

## UNLOCK

This module uses multiple threads to detect whether users can normally use internet services, such as streaming media,
game platforms, etc.

Usage:
```bash
rsbench -u [OPTIONS]
```

Optional parameters:

- `--region [location]`: Specify regions, multiple can be specified, such as `--region cn --region us`.

By default, if no region is specified, internet services in all regions are tested.

**Please note that due to the timeliness and specificity of the unblocking test scripts, the results may not match the
actual situation!**

Example output:
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

Specifying regions:
```bash
rsbench -u --region cn --region us

UNLOCK:
 Y/N             Service             Error
[ Y ]    Bilibili China Mainland
[ Y ]       IQIYI Oversea (HK)
[ N ]            Sling TV            Not available
Tested 3 projects took 2.32 seconds, 2 services unlocked, 1 services locked.
```

Currently classified regions:
```
hk, mo, tw, jp, cn, asia, euro, afr, uk, us, global
```

`global` refers to platforms that provide services globally, such as Netflix (even if it's not supported in very few
regions).

## Common Parameters

- `--help`: Display help information
- `--version`: Display version information
- `--no-color`: Disable color output
- `--no-cls`: Disable clearing the screen, the screen is cleared by default before the program executes
- `--no-logo`: Do not output Ascii Art Logo

By default, you can combine the functions of the four modules together, for example:
```bash
rsbench -ibtu --network --region cn
```

Or run all tests at once:
```bash
rsbench -ibtu
```
