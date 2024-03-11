# jlu-drcom-rs
jlu drcom客户端的rust实现
# TODO LIST
1. 重构
2. 自动检索mac地址和网络配置
# Usage
因为一些兼容性问题，需要手动指定mac地址
```shell
jlu-drcom-rs run -u username -p password -m mac -t timeout(5)
```

# 2023.08.24
吉大校园网似乎升级了，但经过测试这个版本仍然可以使用，虽然在我的manjaro上时不时会出现连接突然中断的问题，估计不是程序的问题

过两天闲下来会重构一下，加上自动检索mac地址和设置的功能

# 2024.03.11

正在重写为 GUI + CLI 版本
