@echo off
REM RB (Rust Bench) 测试示例脚本 - Windows 版本
REM 请根据实际情况修改 URL 和参数

echo === RB 测试示例 ===
echo.

REM 示例 1: 基础 GET 请求测试
echo 示例 1: 基础 GET 请求测试
echo 命令: rb -n 100 -c 10 https://httpbin.org/get
REM rb -n 100 -c 10 https://httpbin.org/get
echo.

REM 示例 2: 高并发测试
echo 示例 2: 高并发测试 (1000 请求, 50 并发)
echo 命令: rb -n 1000 -c 50 https://httpbin.org/get
REM rb -n 1000 -c 50 https://httpbin.org/get
echo.

REM 示例 3: 基于时间的测试
echo 示例 3: 持续 10 秒的压力测试
echo 命令: rb -t 10 -c 20 https://httpbin.org/get
REM rb -t 10 -c 20 https://httpbin.org/get
echo.

REM 示例 4: 启用 KeepAlive
echo 示例 4: 启用 KeepAlive 连接复用
echo 命令: rb -n 500 -c 25 -k https://httpbin.org/get
REM rb -n 500 -c 25 -k https://httpbin.org/get
echo.

REM 示例 5: POST 请求测试
echo 示例 5: POST 请求测试
echo 命令: rb -n 100 -c 10 -p examples\post_data.json -T "application/json" https://httpbin.org/post
REM rb -n 100 -c 10 -p examples\post_data.json -T "application/json" https://httpbin.org/post
echo.

REM 示例 6: 使用自定义 Headers
echo 示例 6: 使用自定义 HTTP Headers
echo 命令: rb -n 100 -c 10 -H "User-Agent: RB-Test/1.0" -H "Accept: application/json" https://httpbin.org/headers
REM rb -n 100 -c 10 -H "User-Agent: RB-Test/1.0" -H "Accept: application/json" https://httpbin.org/headers
echo.

REM 示例 7: 导出结果为 JSON 格式
echo 示例 7: 导出结果为 JSON 格式
echo 命令: rb -n 100 -c 10 -o result.json https://httpbin.org/get
REM rb -n 100 -c 10 -o result.json https://httpbin.org/get
echo.

REM 示例 8: 导出结果为 CSV 格式
echo 示例 8: 导出结果为 CSV 格式
echo 命令: rb -n 100 -c 10 -o result.csv -f csv https://httpbin.org/get
REM rb -n 100 -c 10 -o result.csv -f csv https://httpbin.org/get
echo.

REM 示例 9: 综合测试（高并发 + KeepAlive + 自定义 Headers + 导出）
echo 示例 9: 综合测试（高并发 + KeepAlive + 自定义 Headers + 导出）
echo 命令: rb -n 1000 -c 50 -k -H "User-Agent: RB-Test/1.0" -o report.json https://httpbin.org/get
REM rb -n 1000 -c 50 -k -H "User-Agent: RB-Test/1.0" -o report.json https://httpbin.org/get
echo.

echo 注意: 以上命令已被注释（REM），删除 REM 后即可运行
echo 建议使用 httpbin.org 或您自己的测试服务器进行测试
pause
