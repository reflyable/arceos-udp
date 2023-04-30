# 第八周进度报告

吴大帅
## 本周工作
- 参考rCore和学长已有实现, 将clibax中socket集成进fs, 由fs_table统一管理, 完成相关api
- 尽量按照linux标准完善net的c接口的rust部分
    - 已经全部完成: socket, connect, shutdown, recv, send, recvfrom, sendto
    - 还需最后一层包装: accept, listen
    - 未开始: getaddrinfo

## 主要困难
- 对linux c接口的细节不清楚, 需要细致翻阅文档
- 对rust的最佳实践不熟悉, 需要和学长交流

## 下周计划
- 完成全部接口的rust和c部分
- 实现c语言测例