# Usage Guide

This project consists of four main modules:

- INFO:
  - OS: Operating system information
  - CPU: CPU information
  - MEM: Memory information
  - DISK: Disk information
  - SWAP: Swap partition information
  - KERN: Kernel information
  - VIRT: Virtualization information

  Output information is for reference only and should not be used as a performance benchmark.
- BENCH:
  - FIB: Fibonacci sequence calculation performance test
  - DISK: Disk sequential read/write test (currently buggy and not available)
  - MEM: Memory read/write test
  - PING: Network latency test (Cloudflare TCPing)
  - LOSS: Network packet loss test (Cloudflare TCPing)
  - NET: Network test (limited to connectivity testing with Cloudflare speed test servers; for multi-path testing, see Speedtest below)
    Test results are for reference only and should not be used as a performance benchmark.
- TUNE:
  - Speedtest: Network speed test
- UNLOCK:
  Internet service unlock testing, used to check whether users can access internet services such as streaming platforms, gaming platforms, etc.

## INFO

When executing the binary without any parameters, the `INFO` module is run by default.

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

This module comprehensively evaluates host performance. Currently implemented features include `network testing / CPU performance testing / disk sequential read/write testing`, with more updates to come.

**Note: This module may heavily utilize system resources. Use with caution.**

Usage:
```bash
rsbench -b [OPTIONS]
```

Optional parameters:
- `--network`: Perform network testing
- `--fib`: Perform CPU FIB calculation performance test
- `--disk`: Perform disk sequential read/write test
- `--mem`: Perform memory read/write test

If no parameters are provided, all tests will be executed.

Output: (If only some tests are enabled, not all information will be output.)
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

### Network Testing

Corresponding parameter: `--network`

`PING` tests the TCPing latency to Cloudflare `1.1.1.1:443`, in milliseconds:
```
PING: 1.22 ms
```

This test connects to Cloudflare Speedtest servers to measure download and upload speeds.

The first two lines show single-thread download speeds (marked with 🔽), with the left and right sides representing *average speed* and *maximum speed*, respectively, in Mbps, displayed as 5-digit numbers:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The next two lines show multi-thread download speeds (marked with ⏬), with the left and right sides representing *average speed* and *maximum speed*, respectively, in Mbps, displayed as 5-digit numbers:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Calculation Test

Corresponding parameter: `--fib`

This test uses the CPU to calculate the Fibonacci sequence and measures the time taken and scores.

The benchmark is based on testing with the [MECHREVO蛟龙16 Pro](https://www.mechrevo.com/content/details92_4817.html) specifications. The score is calculated as `benchmark / test time`, with higher scores indicating better performance.

The upper and lower lines show *single-thread score* and *multi-thread score*, respectively:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read/Write Test

Corresponding parameter: `--disk`

This test performs sequential read/write operations on the disk and measures the time taken and scores.

For Windows, the default test file path is `C:\\rsbench_disk_test`; for other systems, it is `/tmp/rsbench_disk_test`.

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
- `--speedtest`: Perform Speedtest

If no parameters are provided, all tests will be executed.

### Speedtest

Corresponding parameter: `--speedtest`

This test uses Speedtest.net / cn for single-thread / multi-thread testing and outputs detailed information.

Optional parameters:

- `--custom-speedtest-host <CUSTOM_SPEEDTEST_HOST>`: Custom Speedtest server address, formatted as `DOMAIN:PORT`, without protocol prefixes like `http`.

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

- `Speedtest.net`: Default speed test server, selecting the nearest server via API.
- `China Mobile`: speedtest1.sc.chinamobile.com:8080
- `China Unicom`: 36.250.1.90:8080
- `China Telecom`: speedtest1.online.sh.cn:8080
- `HK I3D`: hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080
- `TW HiNet`: ntp1.chtm.hinet.net:8080
- `JP xTom`: speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080

For more information, refer to [
`12101111 Blog's article`](https://12101111.github.io/speedtest-net/#https-www-speedtest-net-api-js-servers-engine-js) and
the speed test server collection list: [CN](https://github.com/spiritLHLS/speedtest.cn-CN-ID) / [Net](https://github.com/spiritLHLS/speedtest.net-CN-ID).

## UNLOCK

This module uses multi-threading to check whether users can access internet services such as streaming platforms, gaming platforms, etc.

Usage:
```bash
rsbench -u [OPTIONS]
```

Optional parameters:
- `--region [location]`: Specify a region. Multiple regions can be specified, e.g., `--region cn --region us`.

By default, if no region is specified, all regions' internet services will be tested.

**Note: Due to the time-sensitive and specific nature of unlock testing scripts, results may not always match reality!**

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

Currently categorized regions:
```
hk, mo, tw, jp, cn, asia, euro, afr, uk, us, global
```

`global` refers to platforms available worldwide, such as Netflix (considered available even if unavailable in a few regions).

## Uploading to Pastebin

By default, test results are automatically uploaded to Pastebin after completion. You can control this behavior with the following parameter:

- `--no-upload`: Disable uploading.

The default upload destination is <https://pastebin.highp.ing>, with the project repository at <https://github.com/rsbench/pastebin>.

Due to deployment on Cloudflare Workers and the use of D1 database, which has certain limitations, authentication is required.

For official Github Actions builds, authentication information is embedded, so no additional configuration is needed during installation.

For self-compiled builds, you need to configure authentication information yourself. Refer to this documentation for details (not yet written).

## General Parameters

- `--help`: Display help information.
- `--version`: Display version information.
- `--no-color`: Disable colored output.
- `--no-cls`: Disable screen clearing. By default, the screen is cleared before execution.
- `--no-logo`: Do not output Ascii Art Logo.
- `--no-usage`: Do not submit usage statistics.
- `--no-upload`: Disable uploading test results to Pastebin.

By default, you can combine the functionalities of all four modules, e.g.:
```bash
rsbench -ibtu --network --region cn
```

Or run a full test directly:
```bash
rsbench -ibtu
```