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

## 打包（绿色版 / 安装包）

```powershell
# PowerShell（推荐）
.\packaging\build.ps1
```

```bash
# Git Bash
bash packaging/build.sh
```

产物在 `dist/`：

| 文件 | 说明 |
|------|------|
| `随密-<版本>-win64-绿色版.zip` | 解压即用，含 `随密.exe`、`启动.bat` |
| `随密-<版本>-win64-setup.exe` | 安装包（**NSIS** 或 **Inno Setup** 二选一即可） |

安装打包工具后重跑脚本（优先 NSIS）：

```text
winget install NSIS.NSIS --source winget
# 或
winget install JRSoftware.InnoSetup --source winget
```

详见 [packaging/README.md](packaging/README.md)（含 WiX/MSI 等其它方案）。
