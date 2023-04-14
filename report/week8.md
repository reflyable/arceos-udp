| col1                          | col2                                | col3 |col3| col1                      | col2                                | col3 |
| ----------------------------- | ----------------------------------- | ---- |---| ---------------------------| ----------------------------------- | ---- |
| std::net::TcpListener         | bind                                | √    || std::net::UdpSocket           | bind                                | √   |
|                               | local_addr                          | √    ||                               | recv_from                           | √   |
|                               | try_clone                           |      ||                               | peek_from                           | √   |
|                               | accept                              | √    ||                               | send_to                             | √   |
|                               | incoming                            |      ||                               | peer_addr                           |      |
|                               | (set_)ttl                           |      ||                               | local_addr                          | √   |
|                               | take_error                          |      ||                               | try_clone                           |      |
|                               | set_nonblocking                     |      ||                               | (set_)write/read_timeout            |      |
|                               | fd等类型转化                         |      ||                               | broadcast/multicast_loop相关        |      |
| std::net::TcpStream           | connect                             | √    ||                               | (set_)ttl                           |      |
|                               | connect_timeout                     |      ||                               | take_error                          |      |
|                               | peer_addr                           | √    ||                               | connect相关(connect,recv,send,peek)  |      |
|                               | local_addr                          | √    ||                               | set_nonblocking                     |      |
|                               | shutdown                            | √    ||                               | fd等类型转化                         |      |
|                               | try_clone                           |      || Trait std::net::ToSocketAddrs | 各种类型转化为地址(包括域名查询)       | √   |
|                               | (set_)write/read_timeout            |      |||||
|                               | peek                                |      |||||
|                               | (set_)nodelay                       |      |||||
|                               | (set_)ttl                           |      |||||
|                               | take_error                          |      |||||
|                               | set_nonblocking                     |      |||||
|                               | fd等类型转化                         |      |||||
|                               | Read/Write Trait                    | √    |||||

