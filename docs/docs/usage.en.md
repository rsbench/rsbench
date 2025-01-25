# Usage Guide

This project is divided into four main modules:

- **INFO**:
  Outputs host information such as
  `system environment / CPU / memory / disk / SWAP / kernel / virtualization technology`, etc.

  The output is for reference only and should not be used as a basis for performance judgment.

- **BENCH**:
  Executes benchmark tests, including
  `network testing / CPU performance testing / memory performance testing / disk performance testing`, etc. (some
  features are currently incomplete).

  Test results are for reference only and should not be used as a basis for performance judgment.

- **TUNE**:
  - **IPCheck**: Detects detailed information about an IP.
  - **Speedtest**: Tests network speed.

- **UNLOCK**:
  Tests internet service accessibility, checking if users can access services like streaming media, gaming platforms,
  etc.

## INFO

When executing the binary without any parameters, the `INFO` module is executed by default.

Usage:
```bash
rsbench -i 
```

This function has no additional parameters. After execution, it will output host information:
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

This module comprehensively evaluates host performance. Currently, it includes
`network testing / CPU performance testing / disk sequential read-write testing`, with more features being continuously
updated.

**PS: This module may heavily utilize system resources, so use it with caution.**

Usage:
```bash
rsbench -b [OPTIONS]
```

Optional parameters:

- `--network`: Executes network testing.
- `--fib`: Executes CPU FIB calculation performance testing.
- `--disk`: Executes disk sequential read-write testing.

If no parameters are provided, all tests will be executed.

Output: (If only some tests are enabled, not all information will be output.)
```
PING: 1.22 ms
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Network Testing

Corresponding parameter: `--network`

`PING` tests the TCPing latency to Cloudflare `1.1.1.1:443`, in milliseconds:
```
PING: 1.22 ms
```

This test connects to Cloudflare Speedtest servers to measure download and upload speeds.

The first two lines show single-thread download speeds (marked with 🔽), with the left and right sides showing *average
speed* and *maximum speed*, respectively, in Mbps, displayed with 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The next two lines show multi-thread download speeds (marked with ⏬), with the left and right sides showing *average
speed* and *maximum speed*, respectively, in Mbps, displayed with 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Calculation Testing

Corresponding parameter: `--fib`

This test uses the CPU to calculate the Fibonacci sequence and measures the time taken and the score.

The benchmark is based on tests performed on
the [Mechrevo Jiaolong 16 Pro](https://www.mechrevo.com/content/details92_4817.html). The score is calculated as
`benchmark / test time`, with higher scores indicating better performance.

The output shows *single-thread score* and *multi-thread score*:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read-Write Testing

Corresponding parameter: `--disk`

This test performs sequential read-write operations on the disk and measures the time taken and the score.

For Windows, the default test file path is `C:\\rsbench_disk_test`. For other systems, it is `/tmp/rsbench_disk_test`.

The left side shows write speed, and the right side shows read speed, in MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE

Usage:
```bash
rsbench -t [OPTIONS]
```

Optional parameters:

- `--ip`: Executes network testing.
- `--speedtest`: Executes Speedtest.

If no parameters are provided, all tests will be executed.

### IPCheck

Corresponding parameter: `--ip`

This test detects the user's IP address and outputs detailed information.

The test uses multiple IP service providers. If there are multiple local IPs or corresponding routing rules, multiple
IPs may appear, which is normal.

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

This test uses Speedtest.net / cn for single-thread / multi-thread testing, outputting detailed information.

Optional parameters:

- `--custom-speedtest-host <CUSTOM_SPEEDTEST_HOST>`: Custom Speedtest server address, format `DOMAIN:PORT`, without
  `http` or other protocol prefixes.

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

Default speedtest servers:

- `Speedtest.net`: Default speedtest server, selects the nearest server based on API.
- `China Mobile`: speedtest1.sc.chinamobile.com:8080
- `China Unicom`: 36.250.1.90:8080
- `China Telecom`: speedtest1.online.sh.cn:8080
- `HK I3D`: hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080
- `TW HiNet`: ntp1.chtm.hinet.net:8080
- `JP xTom`: speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080

For more information, refer to [
`12101111 Blog's article`](https://12101111.github.io/speedtest-net/#https-www-speedtest-net-api-js-servers-engine-js)
and
the speedtest server collection
list: [CN](https://github.com/spiritLHLS/speedtest.cn-CN-ID) / [Net](https://github.com/spiritLHLS/speedtest.net-CN-ID)

## UNLOCK

This module multi-threadedly checks if users can access internet services like streaming media, gaming platforms, etc.

Usage:
```bash
rsbench -u [OPTIONS]
```

Optional parameters:

- `--region [location]`: Specifies the region, multiple regions can be specified, e.g., `--region cn --region us`

By default, if no region is specified, all regions' internet services will be tested.

**Please note that due to the timeliness and specificity of the unlock testing script, there may be discrepancies with
actual results!**

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

`global` refers to platforms providing services globally, such as Netflix (even if a few regions are unsupported).

## Upload to Pastebin

By default, test results are automatically uploaded to Pastebin after testing. You can control this behavior with the
following parameter:

- `--no-upload`: Disables upload.

The default upload destination is <https://pastebin.highp.ing>, and the project repository
is: <https://github.com/rsbench/pastebin>

Since it is deployed on Cloudflare Workers with D1 database, there are some limitations, so authentication is required.

In the official Github Actions compilation, authentication information is embedded, so there is no need to worry about
authentication during installation.

If compiling manually, you need to configure authentication information yourself. Please refer to this documentation for
details (not yet written).

## General Parameters

- `--help`: Displays help information.
- `--version`: Displays version information.
- `--no-color`: Disables color output.
- `--no-cls`: Disables screen clearing, which is done by default before program execution.
- `--no-logo`: Does not output Ascii Art Logo.

By default, you can combine the functions of the four modules, for example:
```bash
rsbench -ibtu --network --region cn
```

Or run a full test directly:
```bash
rsbench -ibtu
```