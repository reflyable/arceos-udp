# 操作系统2023春大实验最终报告
吴大帅
## 1.主要工作
### smoltcp和libax
- 将smoltcp UDP和DNS协议栈接入arceos
- 完善了rust std风格的libax rust网络接口
  - UdpSocket 收发包相关接口
  - DNS查询的trait
- 实现、完善了rust udp和dns网络应用及测例
  - udp server
  - http client支持DNS

### clibax网络支持
- 模仿musl-libc在clibax中添加了c标准库风格的网络接口
  - socket, connect, shutdown, recv, send, recvfrom, sendto, accept, listen，getaddrinfo 以及对socket fd的读写等操作
- 重构fd_table，增加其他fd类型的支持
- 实现了c的网络应用测例
  - udp server
  - http client (dns)
  - http server

### iperf3迁移
- 引入[printf嵌入式实现](https://github.com/mpaland/printf)并修改clibax中相关代码以完整支持printf系列函数
- 实现/完善一些库函数
  - select
  - set/getsockopt
  - fcntl
- 修改iperf源代码以绕过某些库函数调用

共进行4次PR，净代码量约5~6千行

## 2.demo
### rust app
- http 客户端（dns）
![http client](https://github.com/reflyable/arceos-udp/assets/71587404/5b9a2703-100b-48a7-93e4-61274a7be178)
- udp 服务端
![udp server](report/image/week8/1681578523233.png)

### C app
- http 客户端（dns）
![http client](report/image/week12/013857.png)
- http 服务端
![http server](report/image/week12/014139.png)

### iperf3
- 收包
![iperf收包](https://github.com/reflyable/arceos-udp/assets/71587404/c7632029-ff26-4ffa-913f-00715851b2cc)
- 发包
![image](https://github.com/reflyable/arceos-udp/assets/71587404/09816ea8-0ebb-4c6c-94e8-4c9502c49e93)
