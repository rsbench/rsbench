# RSBench

A **simple**, **convenient**, **cross-platform**, **high-performance** benchmarking tool written in pure Rust.

Have you been suffering from the lack of good **benchmarking** tools in many server / VPS and other environments?

Dependency issues? Unsupported architectures? Incorrect output formatting?

**[RSBench](https://github.com/rsbench/rsbench) is here to help you!**

## Basic Functions

- INFO Information Output: Outputs machine environment information
- BENCH Performance Testing: Covers CPU, memory, network, disk performance testing and evaluates scores
- TUNE Some Small Functionalities: Speedtest, etc.
- UNLOCK Internet Service Unblock Test: Quickly checks if internet services with IP restrictions are blocked

## Features

- Written in pure Rust, supports multiple platforms and architectures, excellent performance
- Multi-threaded testing
- Color output support
- Highly extensible, for example, the Unlock unblocking test is designed as a module, allowing you to add/modify new
  test modules yourself
- Does not require special privileges (even with restrictions for ordinary users)
- ...

Table of Contents:

1. [Installation](install.zh.md)
2. [Usage](usage.zh.md)