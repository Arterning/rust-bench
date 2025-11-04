# RB (Rust Bench) - HTTP 压力测试工具

一个使用 Rust 编写的高性能 HTTP 压力测试命令行工具，类似于 Apache Bench (ab)。

## 特性

- 支持高并发请求测试
- 支持 GET 和 POST 请求
- 支持自定义 HTTP Headers
- 支持 HTTP KeepAlive 连接复用
- 支持基于请求数或时间的测试模式
- 提供详细的统计信息（响应时间分布、百分位数、QPS 等）
- 支持结果导出（JSON/CSV 格式）
- 彩色输出，易于阅读

## 安装

### 前置要求

- Rust 1.70 或更高版本

### 编译

```bash
# 克隆或下载项目后，进入项目目录
cd rust-bench

# 编译项目（Debug 模式）
cargo build

# 或编译 Release 版本（推荐，性能更好）
cargo build --release

# 运行
./target/release/rb --help
```

### 安装到系统

```bash
# 安装到 ~/.cargo/bin/
cargo install --path .

# 之后可以直接使用
rb --help
```

## 使用方法

```bash
rb [OPTIONS] <URL>
```

### 参数说明

| 参数 | 长参数 | 说明 | 默认值 |
|------|--------|------|--------|
| `-n` | `--requests` | 总请求数 | 100 |
| `-c` | `--concurrency` | 并发数 | 10 |
| `-t` | `--timelimit` | 测试持续时间(秒)，设置后忽略 -n | 无 |
| `-p` | `--postfile` | POST 数据文件路径 | 无 |
| `-T` | `--content-type` | Content-Type 头 | 无 |
| `-H` | `--header` | 添加自定义 Header (可多次使用) | 无 |
| `-k` | `--keepalive` | 启用 HTTP KeepAlive | false |
| `-o` | `--output` | 导出结果到文件 | 无 |
| `-f` | `--format` | 导出格式 (json 或 csv) | json |
| `-h` | `--help` | 显示帮助信息 | - |
| `-V` | `--version` | 显示版本信息 | - |

## 使用示例

### 1. 基础 GET 请求测试

```bash
# 发送 100 个请求，并发数为 10
rb https://example.com

# 自定义请求数和并发数
rb -n 1000 -c 50 https://example.com
```

### 2. 基于时间的测试

```bash
# 持续测试 30 秒
rb -t 30 -c 20 https://example.com
```

### 3. POST 请求测试

创建一个 POST 数据文件 `post_data.json`:
```json
{
  "username": "test",
  "password": "123456"
}
```

然后运行测试：
```bash
# POST 请求，指定 Content-Type
rb -n 500 -c 25 -p post_data.json -T "application/json" https://api.example.com/login
```

### 4. 使用自定义 Headers

```bash
# 添加单个 Header
rb -H "Authorization: Bearer token123" https://api.example.com/data

# 添加多个 Headers
rb -H "Authorization: Bearer token123" \
   -H "X-Custom-Header: value" \
   -H "User-Agent: RB-Tester/1.0" \
   https://api.example.com/data
```

### 5. 启用 KeepAlive

```bash
# 启用 HTTP KeepAlive 以复用连接，提高性能
rb -n 1000 -c 50 -k https://example.com
```

### 6. 结果导出

```bash
# 导出结果为 JSON 格式
rb -n 1000 -c 50 -o result.json https://example.com

# 导出结果为 CSV 格式
rb -n 1000 -c 50 -o result.csv -f csv https://example.com

# 指定导出格式（显式指定 JSON）
rb -n 500 -c 25 -o benchmark_report.json -f json https://api.example.com/test
```

### 7. 综合示例

```bash
# 完整的压力测试：1000 请求，50 并发，启用 KeepAlive，自定义 Headers，导出结果
rb -n 1000 -c 50 -k \
   -H "Authorization: Bearer your_token" \
   -H "Accept: application/json" \
   -o report.json \
   https://api.example.com/endpoint
```

## 输出示例

```
Rust Bench - HTTP 压力测试工具

测试配置:
  目标 URL:     https://example.com
  请求数:       1000
  并发数:       50
  KeepAlive:    启用

开始测试... (共 1000 个请求)

=== 测试报告 ===

请求统计:
  总请求数:     1000
  成功请求:     998 (99.80%)
  失败请求:     2 (0.20%)

时间统计:
  总耗时:       5.432 秒
  平均响应时间: 245.123 毫秒
  最小响应时间: 123.456 毫秒
  最大响应时间: 892.345 毫秒

响应时间百分位:
  P50 (中位数):  234.567 毫秒
  P75:          298.765 毫秒
  P90:          456.789 毫秒
  P95:          567.890 毫秒
  P99:          789.012 毫秒

吞吐量:
  QPS (每秒请求数): 183.75
```

## 导出文件格式

### JSON 格式

使用 `-o result.json` 或 `-o result.json -f json` 导出时，会生成包含以下字段的 JSON 文件：

```json
{
  "total_requests": 1000,
  "successful_requests": 998,
  "failed_requests": 2,
  "success_rate": 99.8,
  "total_duration_secs": 5.432,
  "avg_response_time_ms": 245.123,
  "min_response_time_ms": 123.456,
  "max_response_time_ms": 892.345,
  "p50_ms": 234.567,
  "p75_ms": 298.765,
  "p90_ms": 456.789,
  "p95_ms": 567.890,
  "p99_ms": 789.012,
  "qps": 183.75
}
```

### CSV 格式

使用 `-o result.csv -f csv` 导出时，会生成包含所有统计指标的 CSV 文件，字段与 JSON 格式相同，适合在 Excel 或其他数据分析工具中使用。

CSV 文件示例：
```csv
total_requests,successful_requests,failed_requests,success_rate,total_duration_secs,avg_response_time_ms,min_response_time_ms,max_response_time_ms,p50_ms,p75_ms,p90_ms,p95_ms,p99_ms,qps
1000,998,2,99.8,5.432,245.123,123.456,892.345,234.567,298.765,456.789,567.890,789.012,183.75
```

## 性能提示

1. **使用 Release 编译**:
   ```bash
   cargo build --release
   ```
   Release 版本比 Debug 版本性能提升显著。

2. **启用 KeepAlive**: 使用 `-k` 参数可以复用 TCP 连接，大幅提升性能。

3. **合理设置并发数**: 并发数不是越高越好，建议根据目标服务器性能和网络状况调整。

4. **注意系统限制**: 在高并发场景下，可能需要调整系统的文件描述符限制：
   ```bash
   # Linux/macOS
   ulimit -n 10000
   ```

## 技术栈

- **clap**: 命令行参数解析
- **reqwest**: HTTP 客户端
- **tokio**: 异步运行时
- **anyhow**: 错误处理
- **colored**: 彩色输出
- **serde**: 序列化/反序列化
- **serde_json**: JSON 序列化
- **csv**: CSV 文件处理

## 与 Apache Bench (ab) 的对比

| 特性 | RB | AB |
|------|----|----|
| 并发性能 | 基于 Rust 异步，性能优秀 | 基于 C，性能也很好 |
| 跨平台 | 优秀 | 良好 |
| 安装 | 需要编译 | 通常预装或易于安装 |
| 统计详细度 | 详细（百分位数等） | 详细 |
| KeepAlive | 支持 | 支持 |
| 自定义 Headers | 支持多个 | 支持多个 |
| 结果导出 | 支持 JSON/CSV | 不支持 |

## 注意事项

1. 请不要对不属于你的服务器进行压力测试，这可能被视为 DDoS 攻击
2. 在进行压力测试前，请确保获得相关授权
3. 建议先从小并发开始测试，逐步增加
4. 密切关注目标服务器的响应和资源使用情况

## License

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 待优化功能

- [ ] 支持 HTTPS 证书验证配置
- [ ] 支持请求超时设置
- [ ] 支持实时进度显示
- [x] 支持结果导出（JSON/CSV）✅
- [ ] 支持更多 HTTP 方法（PUT、DELETE 等）
- [ ] 支持从文件批量读取 URL
- [ ] 支持响应内容验证
