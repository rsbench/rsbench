# Usage Guide

This project is divided into four main modules:

- **INFO:**
  - **OS:** Operating system information
  - **CPU:** CPU information
  - **MEM:** Memory information
  - **DISK:** Disk information
  - **SWAP:** Swap partition information
  - **KERN:** Kernel information
  - **VIRT:** Virtualization information
    The output information is for reference only and should not be used as a basis for performance evaluation.

- **BENCH:**
  - **FIB:** Fibonacci sequence computation performance test
  - **DISK:** Disk sequential read/write test (has many bugs and is currently disabled)
  - **MEM:** Memory read/write test
  - **PING:** Network latency test (uses Cloudflare TCPing)
  - **LOSS:** Network packet loss test (uses Cloudflare TCPing)
  - **NET:** Network test (only tests the connection to Cloudflare speed test servers; for comprehensive multi-route
    testing, see Speedtest below)
    Test results are for reference only and should not be used as a basis for performance evaluation.

- **TUNE:**
  - **IPCheck:** Detailed IP information detection
  - **Speedtest:** Network speed test

- **UNLOCK:**
  Internet service unlock test, used to verify whether users can normally use internet services such as streaming media,
  gaming platforms, etc.

## INFO

When running the `Binary` without any parameters, it defaults to executing the `INFO` module.
Usage:
```bash
rsbench -i
```

This feature has no additional parameters. After execution, it will output host information:
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

This module comprehensively evaluates host performance. Currently, it implements functions such as network tests, CPU
performance tests, and disk sequential read/write tests. More features are being continuously added.
**Note:** This module may utilize system resources intensively, so use with caution.
Usage:
```bash
rsbench -b [OPTIONS]
```
Optional parameters:

- `--network`: Execute network tests
- `--fib`: Execute CPU FIB computation performance test
- `--disk`: Execute disk sequential read/write test
- `--mem`: Execute memory read/write test
  If no parameters are provided, all tests will be executed.
  Output: (If only some tests are enabled, not all information will be output)
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

### Network Test
Corresponding parameter: `--network`
`PING` tests the TCPing latency to Cloudflare `1.1.1.1:443`, in milliseconds:
```
PING: 1.22 ms
```

This test connects to the Cloudflare speed test server to test download and upload speeds.
The first two lines show single-thread download speeds (marked with 🔽), with average and maximum speeds on the left and
right respectively, in Mbps, showing 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The last two lines show multi-thread download speeds (marked with ⏬), with average and maximum speeds on the left and
right respectively, in Mbps, showing 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Computation Test
Corresponding parameter: `--fib`
This test uses the CPU to compute the Fibonacci sequence and calculates the time taken and score.
The benchmark is based on testing with the specifications of
the [Mechrevo蛟龙 16 Pro](https://www.mechrevo.com/content/details92_4817.html). The score is calculated as
`benchmark / test duration`. Higher scores indicate better performance.
Top and bottom values represent *single-thread score* and *multi-thread score* respectively:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read/Write Test
Corresponding parameter: `--disk`
This test performs sequential read/write tests on the disk and calculates the time taken and score.
For Windows, the default test file path is `C:\rsbench_disk_test`; for other systems, it is `/tmp/rsbench_disk_test`.
Left value is write speed, right value is read speed, in MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE
Usage:
```bash
rsbench -t [OPTIONS]
```
Optional parameters:

- `--ip`: Execute network test
- `--speedtest`: Execute Speedtest test
  If no parameters are provided, all tests will be executed.

### IPCheck
Corresponding parameter: `--ip`
This test detects the user's IP address and outputs detailed information.
This test uses multiple IP providers to perform the detection. If there are multiple IPs locally or
corresponding分流rules, multiple IP situations may appear, which is normal.
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
Tested 14 projects took 5.84 seconds
```

### Speedtest
Corresponding parameter: `--speedtest`
This test uses Speedtest.net/cn for single-thread/multi-thread testing and outputs detailed information.
Optional parameter:

- `--custom-speedtest-host <CUSTOM_SPEEDTEST_HOST>`: Custom Speedtest server address, format `DOMAIN:PORT`, no need for
  `http` protocol prefix.
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

- `Speedtest.net`: Default speed test server, selects the nearest server based on API.
- `China Mobile`: speedtest1.sc.chinamobile.com:8080
- `China Unicom`: 36.250.1.90:8080
- `China Telecom`: speedtest1.online.sh.cn:8080
- `HK I3D`: hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080
- `TW HiNet`: ntp1.chtm.hinet.net:8080
- `JP xTom`: speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080

For more information, refer to:
`12101111 Blog's post`
on [this link](https://12101111.github.io/speedtest-net/#https-www-speedtest-net-api-js-servers-engine-js) and the speed
test server collection
list: [CN](https://github.com/spiritLHLS/speedtest.cn-CN-ID) / [Net](https://github.com/spiritLHLS/speedtest.net-CN-ID)

## UNLOCK

This module runs multiple threads to check if users can normally use internet services such as streaming media, gaming
platforms, etc.
Usage:
```bash
rsbench -u [OPTIONS]
```
Optional parameters:

- `--region [location]`: Specify regions, multiple regions can be specified, e.g., `--region cn --region us`
  By default, if no region is specified, all regional internet services will be tested.
  **Please note:** Due to the time-sensitive and specific nature of the unlock test scripts, discrepancies with actual
  availability may occur!

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

`global` refers to platforms that provide services globally, such as Netflix (even small regions where it's not
supported are considered).

## Upload to Pastebin

By default, after testing, the results will be automatically uploaded to Pastebin. You can control this behavior with
the following parameters:

- `--no-upload`: Disable upload
  Defaults upload to <https://pastebin.highp.ing>, project at: <https://github.com/rsbench/pastebin>

Since it is deployed on Cloudflare Workers with D1 database, there are some limitations, so authentication is required.
In official GitHub Actions builds, authentication information will be built-in, so there is no need to worry about
authentication issues during installation.
If you compile it yourself, you will need to configure authentication information yourself. Please refer to this
document (not yet written).

## General Parameters

- `--help`: Display help information
- `--version`: Display version information
- `--no-color`: Disable color output
- `--no-cls`: Disable clear screen, clears the screen by default before program execution
- `--no-logo`: Do not output ASCII Art Logo
- `--no-usage`: Do not submit usage statistics
- `--no-upload`: Disable uploading test results to Pastebin

By default, you can combine the functionalities of the four modules together, for example:
```bash
rsbench -ibtu --network --region cn
```

Or run all tests directly:
```bash
rsbench -ibtu
```