# rust-quant-trading

## 系统架构

- 使用Trait定义核心接口
- 采用异步编程模型（async/await）
- 强大的错误处理机制
- 类型安全的数据结构


## 核心组件

- 配置管理：使用强类型配置
- 数据获取：异步数据获取接口
- 策略实现：可扩展的策略trait
- 风险管理：实时风险控制
- 交易执行：异步订单处理

## 技术特点

- 使用tokio作为异步运行时
- 使用serde进行序列化
- 使用rust_decimal处理金融计算
- 使用tracing进行日志记录
- 使用thiserror和anyhow处理错误

## 优势

- 内存安全
- 并发性能好
- 类型安全
- 编译时错误检查

# Compilation and testing

```
cargo run
```


# Configuration

To run the application, you need to provide a configuration file.

1. Copy `config.example.json` to `config.json`.
2. Replace the placeholder values in `config.json` with your actual API key, secret, and other parameters.
3. Alternatively, you can create a `.env` file with the required environment variables.