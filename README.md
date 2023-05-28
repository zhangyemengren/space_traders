# space_traders

## workspace
| 条目     | 用途                      |
|--------|-------------------------|
| api    | space_traders官方API及组合操作 |
| ui     | 可视化当前状态及操作              |
| script | 运行后台任务，定时任务和执行特定策略      |

## 运行测试
```bash
// --test 集成测试模式 mod_name模块名 func_name函数名 -q 不显示cargo 输出 -- --nocapture 显示测试输出
cargo test --test mod_name func_name -q -- --nocapture
// 单元测试
cargo test func_name -q -- --nocapture
```

## 生成文档
```bash
cargo doc --open --no-deps
```

## 更新trunk
```bash
cargo install trunk --force
```

## 第一次运行trunk时因为全局没有安装tailwindcss 需要安装
```bash
npm --registry=https://registry.npmjs.org i -g tailwindcss
```

## 运行web ui
```bash
trunk serve --open
```

### TOKEN 存储在github Environments 本地为api/.env
```