# space_traders

## run tests
```bash
// --test 集成测试模式 mod_name模块名 func_name函数名 -q 不显示cargo 输出 -- --nocapture 显示测试输出
cargo test --test mod_name func_name -q -- --nocapture
// 单元测试
cargo test  func_name -q -- --nocapture
```

## 在需要文档注释说明的地方使用rust 三斜线注释 /// 具体见main.rs 注释
```bash
cargo doc --open --no-deps
```

### TOKEN 存储在github Environments