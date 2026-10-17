# 打包说明

## 一键脚本

```powershell
.\packaging\build.ps1
```

```bash
bash packaging/build.sh
```

## 产物

| 类型 | 文件名 |
|------|--------|
| 绿色版 | `dist/随密-<版本>-win64-绿色版.zip` |
| 安装包 | `dist/随密-<版本>-win64-setup.exe` |

## 安装包工具（任选其一）

脚本默认 **先试 NSIS，再试 Inno**（`INSTALLER=both`）。

| 工具 | 安装 | 产物 | 特点 |
|------|------|------|------|
| **NSIS** | `winget install NSIS.NSIS --source winget` | `.exe` | 开源、脚本灵活、体积小，常用 |
| **Inno Setup** | `winget install JRSoftware.InnoSetup --source winget` | `.exe` | 向导界面成熟、中文语言包全 |
| **WiX + cargo-wix** | `cargo install cargo-wix` | `.msi` | 适合企业部署、组策略 |
| **MSIX** | 需额外配置 | `.msix` | 商店 / 现代打包，Rust 生态支持较少 |

仅使用某一种：

```powershell
$env:INSTALLER = "nsis"   # 或 inno / none
.\packaging\build.ps1
```

```bash
INSTALLER=nsis bash packaging/build.sh
```

## 手动编译安装脚本

- NSIS：`packaging/nsis/setup.nsi`
- Inno：`packaging/inno/setup.iss`

安装包内容与绿色版目录一致（exe、`启动.bat`、`使用说明.txt`）。
