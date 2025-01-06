# Usage Method

This project is divided into four major modules

- INFO:
  Outputs information about the host `system environment / CPU / memory / hard disk / SWAP / kernel / virtualization technology`, etc.

  The output information is for reference only and does not serve as a basis for performance judgment
- BENCH:
  Conducts benchmark tests, including `network test / CPU performance test / memory performance test / hard disk performance test`, etc. (some functions are not yet complete)

  The test results are for reference only and do not serve as a basis for performance judgment
- TUNE:
  - IPCheck: Detects detailed information about the IP
- UNLOCK:
  Internet service unlocking test, used to test whether users can use internet services normally, such as streaming media, gaming platforms, etc.

## INFO

When the Binary is executed without any parameters, the `INFO` module is executed by default

Usage method:
```bash
rsbench -i 
```

This function has no other parameters. After execution, it will output the host information:
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

This module will comprehensively evaluate the host's performance. It has currently implemented functions such as `network test / CPU performance test / disk sequential read and write test`, and more are continuously being updated.

**Note: This module may attempt to occupy system resources, please use with caution**

Usage method:
```bash
rsbench -b [OPTIONS]
```

Optional parameters:
- `--network`: Perform network test
- `--fib`: Perform CPU FIB calculation performance test
- `--disk`: Perform disk sequential read and write test

If no parameters are given, all tests will be executed

Output: (if only some test items are enabled, not all information will be output)
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

`PING` is the TCPing delay test for connecting to Cloudflare `1.1.1.1:443`, in ms:
```
PING: 1.22 ms
```

This test will connect to the Cloudflare Speedtest server to test download and upload speeds

The first two lines are single-threaded download speeds (marked with 🔽), with *average speed* and *maximum speed* on the left and right, in Mbps, showing 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The last two lines are multi-threaded download speeds (marked with ⏬), with *average speed* and *maximum speed* on the left and right, in Mbps, showing 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Calculation Test

Corresponding parameter: `--fib`

This test will use the CPU to calculate the Fibonacci sequence and calculate the time taken and the score

The baseline is the test conducted using the [Mechrevo蛟龙 16 Pro](https://www.mechrevo.com/content/details92_4817.html) specifications, with the score being `baseline / test time`, a higher score indicates better performance

The top and bottom represent *single-threaded score* and *multi-threaded score*:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read and Write Test

Corresponding parameter: `--disk`

This test will use the disk for sequential read and write tests and calculate the time taken and the score

For Windows, the default test file path is `C:\\rsbench_disk_test`; for other systems, it is `/tmp/rsbench_disk_test`.

The left value represents the write speed, and the right value represents the read speed, both in units of MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE

Usage method:
```bash
rsbench -t [OPTIONS]
```

Optional parameters:
- `--ip`: Perform an IP check

### IPCheck

Corresponding parameter: `--ip`

This test will detect the user's IP address and output detailed information.

The test uses multiple IP service providers, so if you have multiple IPs locally or corresponding分流 rules, multiple IPs may appear, which is normal.

Example output:
```bash
IP  :
 Provider  |                    IP                    |             Region             | Risk  | Org
Ipinfo.io                 141.**.**.220                      TW - Taiwan - Neihu          N/A    AS38136 Akari Networks
Ipinfo.io            2409:****:****:9c60::67e              CN - Shanghai - Shanghai       N/A    AS9808 China Mobile Communications Group Co., Ltd.
```

## UNLOCK

This module performs multi-threaded checks to determine whether users can use internet services normally, such as streaming media, gaming platforms, etc.

Usage method:
```bash
rsbench -u [OPTIONS]
```

Optional parameters:
- `--region [location]`: Specify regions, multiple regions can be specified, like `--region cn --region us`

By default, not specifying a region means testing all regional internet services.

**Please note that due to the timeliness and uniqueness of the unlocking test scripts, there might be discrepancies with actual conditions!**

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

`global` refers to platforms that provide services worldwide, such as Netflix (with extremely limited regional exceptions).

## General Parameters

- `--help`: Display help information
- `--version`: Display version information
- `--no-color`: Disable colored output
- `ensure_ascii=False`: Prevents JSON from being escaped to ensure non-ASCII characters are output correctly (this parameter seems out of place here and might be intended for another context)
- `--no-cls`: Disable screen clearing, by default the screen is cleared before program execution
- `--no-logo`: Do not output Ascii Art Logo

By default, you can combine the functionalities of the four modules together, for example:
```bash
rsbench -ibtu --network --region cn
```

Or run all tests directly:
```bash
rsbench -ibtu
```
