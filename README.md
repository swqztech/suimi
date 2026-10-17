# 随密 (suimi)

本地随机 **密码 / 用户名 / 网名** 生成器，界面参考 Passroll。使用系统安全随机源（`OsRng`），不上传网络。

## 构建与运行

```bash
cd suimi
cargo build --release
target\release\suimi.exe
```

或双击 `run.bat`（需先 release 构建）。

## 功能

- **密码**：长度滑块、字符集开关、排除易混字符、强度与熵估计、一次生成六个
- **用户名**：可调长度，字母开头，含数字与下划线
- **网名**：内置中文词库组合
