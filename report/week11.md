# 第十一周进度报告

吴大帅
## 本周工作
- 尽量按照linux标准完善net的c接口(包括rust部分和c部分)
    - 已经全部完成: socket, connect, shutdown, recv, send, recvfrom, sendto, accept, listen
    - 存在问题: getaddrinfo
- 实现c语言net app
    - udp server
    - http client (无dns)
    - http server

## 下周计划
- 完善getaddrinfo
- 添加文档,测例等进行pr