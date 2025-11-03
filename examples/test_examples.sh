#!/bin/bash

# RB (Rust Bench) 测试示例脚本
# 请根据实际情况修改 URL 和参数

echo "=== RB 测试示例 ==="
echo ""

# 示例 1: 基础 GET 请求测试
echo "示例 1: 基础 GET 请求测试"
echo "命令: rb -n 100 -c 10 https://httpbin.org/get"
# rb -n 100 -c 10 https://httpbin.org/get
echo ""

# 示例 2: 高并发测试
echo "示例 2: 高并发测试 (1000 请求, 50 并发)"
echo "命令: rb -n 1000 -c 50 https://httpbin.org/get"
# rb -n 1000 -c 50 https://httpbin.org/get
echo ""

# 示例 3: 基于时间的测试
echo "示例 3: 持续 10 秒的压力测试"
echo "命令: rb -t 10 -c 20 https://httpbin.org/get"
# rb -t 10 -c 20 https://httpbin.org/get
echo ""

# 示例 4: 启用 KeepAlive
echo "示例 4: 启用 KeepAlive 连接复用"
echo "命令: rb -n 500 -c 25 -k https://httpbin.org/get"
# rb -n 500 -c 25 -k https://httpbin.org/get
echo ""

# 示例 5: POST 请求测试
echo "示例 5: POST 请求测试"
echo "命令: rb -n 100 -c 10 -p examples/post_data.json -T 'application/json' https://httpbin.org/post"
# rb -n 100 -c 10 -p examples/post_data.json -T "application/json" https://httpbin.org/post
echo ""

# 示例 6: 使用自定义 Headers
echo "示例 6: 使用自定义 HTTP Headers"
echo "命令: rb -n 100 -c 10 -H 'User-Agent: RB-Test/1.0' -H 'Accept: application/json' https://httpbin.org/headers"
# rb -n 100 -c 10 -H "User-Agent: RB-Test/1.0" -H "Accept: application/json" https://httpbin.org/headers
echo ""

# 示例 7: 综合测试
echo "示例 7: 综合测试（高并发 + KeepAlive + 自定义 Headers）"
echo "命令: rb -n 1000 -c 50 -k -H 'User-Agent: RB-Test/1.0' https://httpbin.org/get"
# rb -n 1000 -c 50 -k -H "User-Agent: RB-Test/1.0" https://httpbin.org/get
echo ""

echo "注意: 以上命令已被注释，取消注释后即可运行"
echo "建议使用 httpbin.org 或您自己的测试服务器进行测试"
