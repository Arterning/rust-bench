# 结果导出功能使用指南

## 概述

RB 支持将压力测试结果导出为 JSON 或 CSV 格式，方便后续数据分析和报告生成。

## 快速开始

### 导出为 JSON（默认格式）

```bash
rb -n 1000 -c 50 -o result.json https://example.com
```

### 导出为 CSV

```bash
rb -n 1000 -c 50 -o result.csv -f csv https://example.com
```

## 参数说明

- `-o, --output <FILE>`: 指定输出文件路径
- `-f, --format <FORMAT>`: 指定导出格式（json 或 csv），默认为 json

## 导出字段说明

所有导出文件都包含以下统计字段：

| 字段名 | 说明 | 单位 |
|--------|------|------|
| `total_requests` | 总请求数 | 个 |
| `successful_requests` | 成功请求数 | 个 |
| `failed_requests` | 失败请求数 | 个 |
| `success_rate` | 成功率 | % |
| `total_duration_secs` | 总耗时 | 秒 |
| `avg_response_time_ms` | 平均响应时间 | 毫秒 |
| `min_response_time_ms` | 最小响应时间 | 毫秒 |
| `max_response_time_ms` | 最大响应时间 | 毫秒 |
| `p50_ms` | P50 百分位（中位数） | 毫秒 |
| `p75_ms` | P75 百分位 | 毫秒 |
| `p90_ms` | P90 百分位 | 毫秒 |
| `p95_ms` | P95 百分位 | 毫秒 |
| `p99_ms` | P99 百分位 | 毫秒 |
| `qps` | 每秒请求数 | 个/秒 |

## JSON 格式示例

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

## CSV 格式示例

CSV 文件包含表头和一行数据，适合在 Excel 或其他数据分析工具中打开：

```csv
total_requests,successful_requests,failed_requests,success_rate,total_duration_secs,avg_response_time_ms,min_response_time_ms,max_response_time_ms,p50_ms,p75_ms,p90_ms,p95_ms,p99_ms,qps
1000,998,2,99.8,5.432,245.123,123.456,892.345,234.567,298.765,456.789,567.890,789.012,183.75
```

## 使用场景

### 1. 自动化测试报告

在 CI/CD 流程中使用 JSON 导出，便于自动解析和生成报告：

```bash
rb -n 10000 -c 100 -o /tmp/api-test-report.json https://api.example.com
```

### 2. 性能对比分析

导出多次测试结果，使用 Excel 或 Python 进行对比分析：

```bash
# 测试不同并发数的性能
rb -n 1000 -c 10 -o result_c10.csv -f csv https://example.com
rb -n 1000 -c 50 -o result_c50.csv -f csv https://example.com
rb -n 1000 -c 100 -o result_c100.csv -f csv https://example.com
```

### 3. 长期性能监控

定期运行压力测试并导出结果，追踪服务性能变化：

```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
rb -n 5000 -c 50 -o "performance_${DATE}.json" https://api.example.com
```

### 4. 数据可视化

使用 Python、R 或其他数据分析工具读取导出文件，生成可视化图表：

```python
import json
import matplotlib.pyplot as plt

# 读取多个测试结果
with open('result1.json') as f:
    data1 = json.load(f)

# 绘制性能对比图
plt.plot([data1['p50_ms'], data1['p95_ms'], data1['p99_ms']])
plt.show()
```

## Python 示例：批量测试和分析

```python
import subprocess
import json
import pandas as pd

# 批量测试不同并发数
concurrencies = [10, 25, 50, 100]
results = []

for c in concurrencies:
    output_file = f"test_c{c}.json"
    subprocess.run([
        "rb", "-n", "1000", "-c", str(c),
        "-o", output_file,
        "https://httpbin.org/get"
    ])

    with open(output_file) as f:
        data = json.load(f)
        data['concurrency'] = c
        results.append(data)

# 转换为 DataFrame 并分析
df = pd.DataFrame(results)
print(df[['concurrency', 'qps', 'avg_response_time_ms', 'p95_ms']])

# 导出汇总报告
df.to_csv('summary_report.csv', index=False)
```

## 注意事项

1. **文件覆盖**: 如果输出文件已存在，会被覆盖，请注意备份重要数据
2. **格式自动识别**: 虽然可以通过 `-f` 参数指定格式，但建议使用合适的文件扩展名（.json 或 .csv）
3. **路径权限**: 确保对输出目录有写入权限
4. **CSV 单行**: CSV 格式只包含一行数据（不包括表头），适合批量追加到汇总文件

## 故障排查

### 问题：无法创建输出文件

**可能原因**:
- 目录不存在
- 没有写入权限

**解决方案**:
```bash
# 确保目录存在
mkdir -p ./reports
rb -n 1000 -c 50 -o ./reports/result.json https://example.com

# 检查权限
ls -la ./reports/
```

### 问题：不支持的导出格式

**错误信息**: `不支持的导出格式: xxx，请使用 'json' 或 'csv'`

**解决方案**: 只支持 `json` 和 `csv` 两种格式，请检查 `-f` 参数

```bash
# 正确
rb -o result.json -f json https://example.com
rb -o result.csv -f csv https://example.com

# 错误
rb -o result.xml -f xml https://example.com
```

## 更多示例

查看 `examples/` 目录下的示例文件：
- `sample_result.json` - JSON 格式示例
- `sample_result.csv` - CSV 格式示例
- `test_examples.sh` - 测试脚本示例（Linux/Mac）
- `test_examples.bat` - 测试脚本示例（Windows）
