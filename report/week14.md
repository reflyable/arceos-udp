# 第十四周进度报告

吴大帅
## 本周工作
将Iperf3迁移到Arceos，可以通过iperf_api正常使用
基本未更改iperf3源代码，只是注释/用0替换了部分获取硬件信息的函数，将头文件引用修改为clibax，替换了不必要的复杂函数
经测试可以进行server和client的连接，作为server进行udp连接时需要在client指定-l=1300(性能较佳), 大于14xx会导致smoltcp丢弃而收不到udp包。
主要更改:
- 引入嵌入式printf实现来支持更完全的vsnprintf及snprintf
- 支持若干基本的浮点数库函数
- 实现select，setsockopt库函数
![image](https://github.com/reflyable/arceos-udp/assets/71587404/d5140c8b-ba75-4e04-a897-da10dc9944c0)

## 下周计划
- 完成PR
