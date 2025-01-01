# Usage

This project is divided into four major modules:

- INFO:
  Outputs information about the host's `System Environment / CPU / Memory / Disk / SWAP / Kernel / Virtualization Technology`, etc.

  The output information is for reference only and should not be used as a basis for performance evaluation.
- BENCH:
  Performs benchmark tests, including `Network Testing / CPU Performance Testing / Memory Performance Testing / Disk Performance Testing`, etc. (Some features are currently incomplete)

  Test results are for reference only and should not be used as a basis for performance evaluation.
- TUNE:
  This part is completely unfinished and cannot be used temporarily.
- UNLOCK:
  Internet service unlock testing, used to test whether users can normally use internet services such as streaming media, gaming platforms, etc.

## INFO

When executing the Binary without passing any parameters, it defaults to running the `INFO` module.

Usage:
```bash
> rsbench -i 
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

This module comprehensively evaluates host performance. Currently, it has implemented functions like `network testing / CPU performance testing / disk sequential read and write testing`, with more updates ongoing.

**PS: This module may fully utilize system resources, please use it cautiously**

Usage:
```bash
> rsbench -b [OPTIONS]
```

Optional Parameters:
- `--network`: Perform network testing
- `--fib`: Perform CPU FIB operation performance testing
- `--disk`: Perform disk sequential read/write testing

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

### Network Testing

Corresponding Parameter: `--network`

`PING` represents the TCPing latency to Cloudflare `1.1.1.1:443`, in units of ms:
```
PING: 1.22 ms
```

This test connects to the Cloudflare Speedtest server to test download speed and upload speed.

The first two lines represent single-threaded download speeds (marked by 🔽), with the left and right sides showing *average speed* and *maximum speed*, respectively, in Mbps, displaying 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The last two lines represent multi-threaded download speeds (marked by ⏬), with the left and right sides showing *average speed* and *maximum speed*, respectively, in Mbps, displaying 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Operation Testing

Corresponding Parameter: `--fib`

This test uses the CPU to calculate the Fibonacci sequence and measures the time taken as well as the score.

The benchmark is based on testing using the specifications of the [Mech-Revolution Dragon 16 Pro](https://www.mechrevo.com/content/details92_4817.html). The score is calculated as `benchmark / test time`, with higher scores indicating better performance.

The upper and lower figures represent *single-threaded score* and *multi-threaded score*, respectively:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

### Disk Sequential Read/Write Testing

Corresponding Parameter: `--disk`

This test performs sequential read/write tests on the disk and calculates the time taken as well as the score.

For Windows, the default test file storage path is `C:\\rsbench_disk_test`; for other systems, it is `/tmp/rsbench_disk_test`.

The left side shows the write speed, and the right side shows the read speed, both in units of MB/s:
```bash
DISK: 1102.68 MB/s | 2129.57 MB/s
```

## TUNE

This module is temporarily unavailable as it has not been completed yet.

## UNLOCK

This module detects whether the user can normally use internet services such as streaming media, gaming platforms, etc., in multiple threads.

Usage:
```bash
rsbench -u [OPTIONS]
```

Optional Parameters:
- `--region [location]`: Specifies regions, multiple can be specified, e.g., `--region cn --region us`

By default, if no region is specified, it will test internet services for all regions.

**Please note that due to the timeliness and specificity of the unlock test scripts, there may be discrepancies with actual conditions!**

Example Output:
```bash
> rsbench -u
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
> rsbench -u --region cn --region us
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

`global` refers to platforms available globally, such as Netflix (which is unsupported in very few regions).

## General Parameters

- `--help`: Displays help information
- `--version`: Displays version information
- `--no-color`: Disables color output
- `--no-cls`: Disables screen clearing, which occurs by default before program execution.