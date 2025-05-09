# [RSBench](https://github.com/rsbench/rsbench)

一个用纯 Rust 编写的 **简单**、**方便**、**跨平台**、**高性能** 基准测试工具

你是否已经忍受了许多 服务器 / VPS 等环境无好用的**基准测试**工具？

依赖不支持？架构没有？格式化输出错误？

**[RSBench](https://github.com/rsbench/rsbench) 来助你！**

## 基础功能
- INFO 信息输出: 输出机器环境信息
- BENCH 性能测试: 涵盖 CPU、内存、网络、磁盘等性能测试，并评估分数
- TUNE 一些小功能: Speedtest 等
- UNLOCK 互联网服务解锁测试: 快速检测互联网部分限定 IP 的服务是否被限制

## 特性
- 纯 Rust 编写，多平台多架构支持，性能优异
- 多线程测试
- 颜色输出支持
- 可拓展性高，例如 Unlock 解锁测试设计成模块化，可自行添加 / 更改新的测试模块
- 不需要特权 (即使普通用户有限制)
- ...

目录:

1. [安装](install.zh.md)
2. [使用](usage.zh.md)