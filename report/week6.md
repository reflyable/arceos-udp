# 第六周进度汇报

> 吴大帅

## 实验选题

组件化OS--aceros的改进: udp与dns，对应助教：贾跃凯

- 支持udp协议，进一步支持dns实现域名解析
- 运行ping之类网络应用
- 支持通过域名访问网站

## 进度

- 熟悉aceros，阅读并运行net app
- 阅读aceros与smoltcp交互部分代码: axnet
  - smoltcp：rust TCP/IP栈，支持TCP，UDP
  - axnet：DeviceWrapper，InterfaceWrapper等包装下层网卡驱动，SocketSetWrapper等包装smoltcp，对上层提供TcpSocket
  - 对udp的支持主要是接入somltcp的udp
- 阅读smoltcp部分接口与结构代码: SocketHandle

## 下周进度

- 阅读smoltcp中udp部分
- 开始尝试接入smoltcp udp栈
