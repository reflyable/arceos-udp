# 第八周进度报告

吴大帅

## 本周工作
- 完善udp/dns部分工作, 修正代码格式, 添加文档, 合并PR
- 阅读c_libax代码, 阅读musl-libc部分代码, 明确clibax net完善方向
- 开始进行clibax net完善

## 下周工作
- 递归完善clibax
    - socket与文件系统
    ```rust
    static FD_TABLE: Mutex<[Option<Arc<Mutex<File>>>; FILE_LIMIT]> = Mutex::new([FD_NONE; FILE_LIMIT]);
    ```
    - 抽象出udp和tcp的共同trait
        - udp的connect
            - udp的并行支持


