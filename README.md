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

## tips
- token目前是写死的


## todo list from 2023.6.7
- [x] wasm-bindgen console 支持JsValue
- [x] serde转换 处理api返回的数据 键值对显示
- [x] leptos 列表
- [x] ui 更新
- [x] 参数从表单获取
- [ ] table
- [ ] 分页器
- [ ] 移动船