use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

#[derive(Parser, Debug)]
#[command(name = "rb")]
#[command(author, version, about = "Rust Bench - HTTP 压力测试工具", long_about = None)]
struct Args {
    /// 目标 URL
    #[arg(value_name = "URL")]
    url: String,

    /// 总请求数
    #[arg(short = 'n', long, default_value = "100")]
    requests: usize,

    /// 并发数
    #[arg(short = 'c', long, default_value = "10")]
    concurrency: usize,

    /// 测试持续时间(秒)，如果设置则忽略 -n 参数
    #[arg(short = 't', long)]
    timelimit: Option<u64>,

    /// POST 数据文件路径
    #[arg(short = 'p', long)]
    postfile: Option<PathBuf>,

    /// 代理服务器 URL (支持 http://, socks4://, socks5://)
    #[arg(short = 'x', long)]
    proxy: Option<String>,

    /// Content-Type 头
    #[arg(short = 'T', long)]
    content_type: Option<String>,

    /// 自定义 Header，格式: "Key: Value"，可多次使用
    #[arg(short = 'H', long)]
    header: Vec<String>,

    /// 启用 HTTP KeepAlive
    #[arg(short = 'k', long)]
    keepalive: bool,

    /// 导出结果到文件
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// 导出格式: json 或 csv
    #[arg(short = 'f', long, default_value = "json")]
    format: String,
}

#[derive(Debug, Clone)]
struct RequestResult {
    success: bool,
    status_code: Option<u16>,
    duration: Duration,
    error: Option<String>,
}

#[derive(Debug)]
struct Stats {
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    total_duration: Duration,
    response_times: Vec<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkReport {
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    success_rate: f64,
    total_duration_secs: f64,
    avg_response_time_ms: f64,
    min_response_time_ms: f64,
    max_response_time_ms: f64,
    p50_ms: f64,
    p75_ms: f64,
    p90_ms: f64,
    p95_ms: f64,
    p99_ms: f64,
    qps: f64,
}

impl Stats {
    fn new() -> Self {
        Stats {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_duration: Duration::ZERO,
            response_times: Vec::new(),
        }
    }

    fn add_result(&mut self, result: &RequestResult) {
        self.total_requests += 1;
        if result.success {
            self.successful_requests += 1;
            self.response_times.push(result.duration);
        } else {
            self.failed_requests += 1;
        }
    }

    fn calculate_percentile(&self, percentile: f64) -> Duration {
        if self.response_times.is_empty() {
            return Duration::ZERO;
        }
        let mut sorted = self.response_times.clone();
        sorted.sort();
        let index = ((percentile / 100.0) * sorted.len() as f64).ceil() as usize - 1;
        sorted[index.min(sorted.len() - 1)]
    }

    fn print_report(&self) {
        println!("\n{}", "=== 测试报告 ===".bright_cyan().bold());
        println!();

        println!("{}", "请求统计:".bright_yellow());
        println!("  总请求数:     {}", self.total_requests);
        println!("  成功请求:     {} ({:.2}%)",
            self.successful_requests,
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        );
        println!("  失败请求:     {} ({:.2}%)",
            self.failed_requests,
            (self.failed_requests as f64 / self.total_requests as f64) * 100.0
        );
        println!();

        if !self.response_times.is_empty() {
            println!("{}", "时间统计:".bright_yellow());
            println!("  总耗时:       {:.3} 秒", self.total_duration.as_secs_f64());

            let avg = self.response_times.iter().sum::<Duration>() / self.response_times.len() as u32;
            let min = self.response_times.iter().min().unwrap();
            let max = self.response_times.iter().max().unwrap();

            println!("  平均响应时间: {:.3} 毫秒", avg.as_secs_f64() * 1000.0);
            println!("  最小响应时间: {:.3} 毫秒", min.as_secs_f64() * 1000.0);
            println!("  最大响应时间: {:.3} 毫秒", max.as_secs_f64() * 1000.0);
            println!();

            println!("{}", "响应时间百分位:".bright_yellow());
            println!("  P50 (中位数):  {:.3} 毫秒", self.calculate_percentile(50.0).as_secs_f64() * 1000.0);
            println!("  P75:          {:.3} 毫秒", self.calculate_percentile(75.0).as_secs_f64() * 1000.0);
            println!("  P90:          {:.3} 毫秒", self.calculate_percentile(90.0).as_secs_f64() * 1000.0);
            println!("  P95:          {:.3} 毫秒", self.calculate_percentile(95.0).as_secs_f64() * 1000.0);
            println!("  P99:          {:.3} 毫秒", self.calculate_percentile(99.0).as_secs_f64() * 1000.0);
            println!();

            let qps = self.successful_requests as f64 / self.total_duration.as_secs_f64();
            println!("{}", "吞吐量:".bright_yellow());
            println!("  QPS (每秒请求数): {:.2}", qps);
        }
    }

    fn to_report(&self) -> BenchmarkReport {
        let avg = if !self.response_times.is_empty() {
            self.response_times.iter().sum::<Duration>().as_secs_f64() * 1000.0
                / self.response_times.len() as f64
        } else {
            0.0
        };

        let min = self
            .response_times
            .iter()
            .min()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0);

        let max = self
            .response_times
            .iter()
            .max()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0);

        BenchmarkReport {
            total_requests: self.total_requests,
            successful_requests: self.successful_requests,
            failed_requests: self.failed_requests,
            success_rate: (self.successful_requests as f64 / self.total_requests as f64) * 100.0,
            total_duration_secs: self.total_duration.as_secs_f64(),
            avg_response_time_ms: avg,
            min_response_time_ms: min,
            max_response_time_ms: max,
            p50_ms: self.calculate_percentile(50.0).as_secs_f64() * 1000.0,
            p75_ms: self.calculate_percentile(75.0).as_secs_f64() * 1000.0,
            p90_ms: self.calculate_percentile(90.0).as_secs_f64() * 1000.0,
            p95_ms: self.calculate_percentile(95.0).as_secs_f64() * 1000.0,
            p99_ms: self.calculate_percentile(99.0).as_secs_f64() * 1000.0,
            qps: self.successful_requests as f64 / self.total_duration.as_secs_f64(),
        }
    }

    fn export_json(&self, path: &PathBuf) -> Result<()> {
        let report = self.to_report();
        let json = serde_json::to_string_pretty(&report)
            .context("序列化 JSON 失败")?;

        let mut file = File::create(path)
            .context(format!("创建文件失败: {}", path.display()))?;

        file.write_all(json.as_bytes())
            .context("写入 JSON 文件失败")?;

        Ok(())
    }

    fn export_csv(&self, path: &PathBuf) -> Result<()> {
        let report = self.to_report();
        let mut wtr = csv::Writer::from_path(path)
            .context(format!("创建 CSV 文件失败: {}", path.display()))?;

        wtr.serialize(&report)
            .context("序列化 CSV 失败")?;

        wtr.flush()
            .context("写入 CSV 文件失败")?;

        Ok(())
    }

    fn export(&self, path: &PathBuf, format: &str) -> Result<()> {
        match format.to_lowercase().as_str() {
            "json" => self.export_json(path),
            "csv" => self.export_csv(path),
            _ => anyhow::bail!("不支持的导出格式: {}，请使用 'json' 或 'csv'", format),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", "Rust Bench - HTTP 压力测试工具".bright_green().bold());
    println!();

    // 验证参数
    validate_args(&args)?;

    // 打印测试配置
    print_config(&args);

    // 运行基准测试
    let stats = run_benchmark(&args).await?;

    // 打印报告
    stats.print_report();

    // 导出结果
    if let Some(ref output_path) = args.output {
        println!();
        println!("正在导出结果到 {} ...", output_path.display());
        stats.export(output_path, &args.format)?;
        println!("{}", format!("✓ 结果已成功导出到: {}", output_path.display()).bright_green());
    }

    Ok(())
}

fn validate_args(args: &Args) -> Result<()> {
    if args.concurrency == 0 {
        anyhow::bail!("并发数必须大于 0");
    }

    if args.timelimit.is_none() && args.requests == 0 {
        anyhow::bail!("请求数必须大于 0");
    }

    if let Some(ref path) = args.postfile {
        if !path.exists() {
            anyhow::bail!("POST 数据文件不存在: {}", path.display());
        }
    }

    // 验证代理 URL 格式
    if let Some(ref proxy_url) = args.proxy {
        if !proxy_url.starts_with("http://")
            && !proxy_url.starts_with("https://")
            && !proxy_url.starts_with("socks4://")
            && !proxy_url.starts_with("socks5://") {
            anyhow::bail!(
                "代理 URL 格式不正确。支持的格式: http://, https://, socks4://, socks5://\n示例: socks5://127.0.0.1:1080"
            );
        }
    }

    Ok(())
}

fn print_config(args: &Args) {
    println!("{}", "测试配置:".bright_yellow());
    println!("  目标 URL:     {}", args.url);

    if let Some(timelimit) = args.timelimit {
        println!("  持续时间:     {} 秒", timelimit);
    } else {
        println!("  请求数:       {}", args.requests);
    }

    println!("  并发数:       {}", args.concurrency);
    println!("  KeepAlive:    {}", if args.keepalive { "启用" } else { "禁用" });

    if let Some(ref proxy) = args.proxy {
        println!("  代理服务器:   {}", proxy);
    }

    if let Some(ref content_type) = args.content_type {
        println!("  Content-Type: {}", content_type);
    }

    if !args.header.is_empty() {
        println!("  自定义 Headers: {}", args.header.len());
    }

    println!();
}

async fn run_benchmark(args: &Args) -> Result<Stats> {
    // 构建 HTTP 客户端
    let client = build_client(args)?;

    // 读取 POST 数据
    let post_data = if let Some(ref path) = args.postfile {
        Some(std::fs::read_to_string(path)
            .context("读取 POST 数据文件失败")?)
    } else {
        None
    };

    let mut stats = Stats::new();
    let start_time = Instant::now();

    // 根据是否设置了时间限制来决定运行模式
    if let Some(timelimit) = args.timelimit {
        println!("开始测试... (持续 {} 秒)", timelimit);
        stats = run_time_limited(
            client,
            args,
            post_data.as_deref(),
            Duration::from_secs(timelimit),
        ).await?;
    } else {
        println!("开始测试... (共 {} 个请求)", args.requests);
        stats = run_request_limited(
            client,
            args,
            post_data.as_deref(),
            args.requests,
        ).await?;
    }

    stats.total_duration = start_time.elapsed();

    Ok(stats)
}

fn build_client(args: &Args) -> Result<reqwest::Client> {
    let mut client_builder = reqwest::Client::builder()
        .pool_max_idle_per_host(if args.keepalive { args.concurrency } else { 0 })
        .pool_idle_timeout(if args.keepalive { Some(Duration::from_secs(90)) } else { None });

    // 配置代理
    if let Some(ref proxy_url) = args.proxy {
        let proxy = reqwest::Proxy::all(proxy_url)
            .context(format!("无法解析代理 URL: {}", proxy_url))?;
        client_builder = client_builder.proxy(proxy);
    }

    // 解析自定义 headers
    let mut headers = reqwest::header::HeaderMap::new();
    for header in &args.header {
        let parts: Vec<&str> = header.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            headers.insert(
                reqwest::header::HeaderName::from_bytes(key.as_bytes())?,
                reqwest::header::HeaderValue::from_str(value)?,
            );
        }
    }

    if !headers.is_empty() {
        client_builder = client_builder.default_headers(headers);
    }

    Ok(client_builder.build()?)
}

async fn run_request_limited(
    client: reqwest::Client,
    args: &Args,
    post_data: Option<&str>,
    total_requests: usize,
) -> Result<Stats> {
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let client = Arc::new(client);
    let mut handles = vec![];

    for _ in 0..total_requests {
        let permit = semaphore.clone().acquire_owned().await?;
        let client = client.clone();
        let url = args.url.clone();
        let content_type = args.content_type.clone();
        let post_data = post_data.map(String::from);

        let handle = tokio::spawn(async move {
            let result = execute_request(client.as_ref(), &url, content_type.as_deref(), post_data.as_deref()).await;
            drop(permit);
            result
        });

        handles.push(handle);
    }

    let mut stats = Stats::new();
    for handle in handles {
        let result = handle.await?;
        stats.add_result(&result);
    }

    Ok(stats)
}

async fn run_time_limited(
    client: reqwest::Client,
    args: &Args,
    post_data: Option<&str>,
    duration: Duration,
) -> Result<Stats> {
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let client = Arc::new(client);
    let start = Instant::now();
    let mut handles = vec![];

    while start.elapsed() < duration {
        let permit = semaphore.clone().acquire_owned().await?;
        let client = client.clone();
        let url = args.url.clone();
        let content_type = args.content_type.clone();
        let post_data = post_data.map(String::from);

        let handle = tokio::spawn(async move {
            let result = execute_request(client.as_ref(), &url, content_type.as_deref(), post_data.as_deref()).await;
            drop(permit);
            result
        });

        handles.push(handle);
    }

    let mut stats = Stats::new();
    for handle in handles {
        let result = handle.await?;
        stats.add_result(&result);
    }

    Ok(stats)
}

async fn execute_request(
    client: &reqwest::Client,
    url: &str,
    content_type: Option<&str>,
    post_data: Option<&str>,
) -> RequestResult {
    let start = Instant::now();

    let request = if let Some(data) = post_data {
        let mut req = client.post(url);
        if let Some(ct) = content_type {
            req = req.header(reqwest::header::CONTENT_TYPE, ct);
        }
        req.body(data.to_string())
    } else {
        client.get(url)
    };

    match request.send().await {
        Ok(response) => {
            let status = response.status();
            let duration = start.elapsed();
            RequestResult {
                success: status.is_success(),
                status_code: Some(status.as_u16()),
                duration,
                error: None,
            }
        }
        Err(e) => {
            let duration = start.elapsed();
            RequestResult {
                success: false,
                status_code: None,
                duration,
                error: Some(e.to_string()),
            }
        }
    }
}
