# 第十三周进度报告

吴大帅
## 本周工作
- 接下来工作: 将iperf3至少一部分功能迁移到arceos, 实现与本地iperf3的互操作
- 阅读iperf3源码
- 实现未支持的c api: 简单起见仅支持用到的功能
  - select
  - set/getsockopt
  - fcntl
  - 更完善的sprintf
  - 几个浮点数api
- 修改iperf3源码绕过某些api的调用

## 下周计划
- 继续迁移
