# Usage

This project is divided into four main modules:

- INFO:
  Outputs information about the host's `system environment / CPU / memory / disk / SWAP / kernel / virtualization technology`, etc.

  The information provided is for reference only and should not be used as a basis for performance assessment.
- BENCH:
  Conducts benchmark tests, including `network testing / CPU performance testing / memory performance testing / disk performance testing`, etc. (some functions are not yet complete)

  The test results are for reference only and should not be used as a basis for performance assessment.
- TUNE:
  This part is not at all complete and is temporarily unusable.
- UNLOCK:
  Internet service unlocking test, used to test whether users can normally use internet services, such as streaming media, gaming platforms, etc.

## INFO

When the Binary is executed without any parameters, the `INFO` module is executed by default.

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

This module will comprehensively evaluate the host's performance. It has currently implemented functions such as `network testing / CPU performance testing`, and more are continuously being updated.

**Note: This module may attempt to fully utilize system resources, please use with caution.**

Usage:
```bash
> rsbench -b [OPTIONS]
```


Optional parameters:
- `--network`: Perform network testing
- `--fib`: Perform CPU FIB computation performance testing

If no parameters are provided, all tests will be executed

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

### Network Testing

Corresponding parameter: `--network`

`PING` is the TCPing latency test for connecting to Cloudflare `1.1.1.1:443`, in milliseconds:
```
PING: 1.22 ms
```

This test will connect to the Cloudflare Speedtest server to test download and upload speeds

The first two lines represent single-threaded download speed (marked with 🔽), with *average speed* and *maximum speed* on the left and right, in Mbps, displaying 5 digits:
```
DOWN: 🔽 68.39 Mbps | MAX : 95.24 Mbps
UP  : 🔼 39.66 Mbps | MAX : 79.75 Mbps
```

The last two lines represent multi-threaded download speed (marked with ⏬), with *average speed* and *maximum speed* on the left and right, in Mbps, displaying 5 digits:
```
DOWN: ⏬ 97.25 Mbps | MAX : 171.4 Mbps
UP  : ⏫ 48.91 Mbps | MAX : 113.6 Mbps
```

### FIB Computation Test

Corresponding parameter: `--fib`

This test will use the CPU to compute the Fibonacci sequence and calculate the time taken and the score

The baseline is using the Azure cloud server Standard_B1s specification for testing, with the score calculated as `baseline / test time`, a higher score indicates better performance

The top and bottom represent *single-threaded score* and *multi-threaded score* respectively:
```
FIB : 0.9673818 (9136ms)
FIB16: 7.3711944 (19455ms total)
```

## TUNE

This module is not yet complete and is temporarily unusable.

## UNLOCK

This module will multi-threadedly detect whether users can normally use internet services, such as streaming media, gaming platforms, etc.

Usage:
```bash
rsbench -u [OPTIONS]
```

Optional parameters:
- `--region [location]`: Specify the region, multiple can be specified, such as `--region cn --region us`

By default, if no region is specified, all regions' internet services will be tested.

**
Please note that due to the certain timeliness and particularity of the streaming unlock test script, it may not match the actual situation!**

Example output:
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

Specify region:
```bash
> rsbench -u --region cn --region us
UNLOCK:
 Y/N             Service             Error
[ Y ]    Bilibili China Mainland    
[ Y ]       IQIYI Oversea (HK)      
[ N ]            Sling TV            Not available
Tested 3 projects took 2.32 seconds, 2 services unlocked, 1 service locked.
```

Currently classified regions:
```
hk, mo, tw, jp, cn, asia, euro, afr, uk, us, global
```

`global` refers to platforms that provide services globally, such as Netflix (considered unavailable in very few regions).

## General Parameters

- `--help`: Display help information
- `--version`: Display version information
- `--no-color`: Disable color output
- `--no-cls`: Disable screen clearing, by default, the screen is cleared before the program executes.