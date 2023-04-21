# 第七周进度汇报

吴大帅

## 实验准备/前期分析

- 结合文档基本理解了smoltcp socket层的工作机制与和下层的配合

  - socket层维护收发缓冲区与(tcp)状态
  - interface层将数据包存入/取出socket
- 了解了tcp/udp的socket流程

  [![1680964778907](image/week7/1680964778907.png)]()
  [![1680964778907](https://pic1.zhimg.com/80/v2-5be3e580d88b27368631a0d3384b1cfc_720w.webp)]()
- 基本理解了已完成的tcp部分的机制与思路
- 简要了解了qemu的网络连接机制

## 实践情况

- 基本完成udp栈的基本功能
  - 模仿std::net::UdpSocket
  - bind, recv_from, send_to, local_addr
- demo



## 下周工作
- 完善udp api/dns

## 问题

- udp应该提供到哪种程度的api
- 进一步的工作
    - 完善axnet/libax::net?
        - 协助接入lwip?
  - 支持dns查询?
  - 模仿linux提供gethostbyname之类的api?
  - dns应用?