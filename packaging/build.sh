#!/usr/bin/env bash
# INSTALLER=nsis|inno|both|none（默认 both）
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PACKAGING="$ROOT/packaging"
STAGE="$PACKAGING/stage"
DIST="$ROOT/dist"
INSTALLER_MODE="${INSTALLER:-both}"

VERSION="$(grep -E '^\s*version\s*=' "$ROOT/Cargo.toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"

echo "==> cargo build --release"
(cd "$ROOT" && cargo build --release)

EXE_SRC="$ROOT/target/release/suimi.exe"
[[ -f "$EXE_SRC" ]] || { echo "缺少 $EXE_SRC"; exit 1; }

PORTABLE_NAME="随密-${VERSION}-win64-绿色版"
PORTABLE_DIR="$DIST/$PORTABLE_NAME"
rm -rf "$DIST"
mkdir -p "$PORTABLE_DIR"

cp -f "$EXE_SRC" "$PORTABLE_DIR/随密.exe"
cp -f "$STAGE/启动.bat" "$STAGE/使用说明.txt" "$PORTABLE_DIR/"

(cd "$DIST" && tar -a -cf "${PORTABLE_NAME}.zip" "$(basename "$PORTABLE_DIR")")
echo "已生成: $DIST/${PORTABLE_NAME}.zip"

INSTALLER_STAGE="$PACKAGING/installer-stage"
rm -rf "$INSTALLER_STAGE"
mkdir -p "$INSTALLER_STAGE"
cp -r "$PORTABLE_DIR"/* "$INSTALLER_STAGE/"

# NSIS 命令行对中文路径易乱码，安装包先用 ASCII 文件名，成功后重命名
SETUP_EXE_ASCII="$DIST/Suimi-${VERSION}-win64-setup.exe"
SETUP_EXE="$DIST/随密-${VERSION}-win64-setup.exe"
BUILT=0

find_makensis() {
  for p in \
    "/c/Program Files (x86)/NSIS/makensis.exe" \
    "/c/Program Files/NSIS/makensis.exe"
  do [[ -f "$p" ]] && echo "$p" && return 0; done
  return 1
}

if [[ "$INSTALLER_MODE" == "nsis" || "$INSTALLER_MODE" == "both" ]]; then
  if MAKENSIS="$(find_makensis)"; then
    echo "==> NSIS: $MAKENSIS"
    WIN_STAGE="$(cygpath -w "$INSTALLER_STAGE")"
    WIN_OUT="$(cygpath -w "$SETUP_EXE_ASCII")"
    WIN_NSI="$(cygpath -w "$PACKAGING/nsis/setup.nsi")"
    "$MAKENSIS" "//INPUTCHARSET" "UTF8" "//DVERSION=$VERSION" "//DSTAGE_DIR=$WIN_STAGE" "//DOUTFILE=$WIN_OUT" "$WIN_NSI"
    mv -f "$SETUP_EXE_ASCII" "$SETUP_EXE"
    echo "已生成 (NSIS): $SETUP_EXE"
    BUILT=1
  fi
fi

if [[ $BUILT -eq 0 && ( "$INSTALLER_MODE" == "inno" || "$INSTALLER_MODE" == "both" ) ]]; then
  for p in \
    "/c/Program Files (x86)/Inno Setup 6/ISCC.exe" \
    "/c/Program Files/Inno Setup 6/ISCC.exe"
  do
    if [[ -f "$p" ]]; then
      echo "==> Inno Setup: $p"
      "$p" "$(cygpath -w "$PACKAGING/inno/setup.iss")" \
        "//DMyAppVersion=$VERSION" \
        "//DStageDir=$(cygpath -w "$INSTALLER_STAGE")" \
        "//DOutputDir=$(cygpath -w "$DIST")"
      echo "已生成 (Inno): $SETUP_EXE"
      BUILT=1
      break
    fi
  done
fi

if [[ $BUILT -eq 0 && "$INSTALLER_MODE" != "none" ]]; then
  echo "未生成安装包。可安装: winget install NSIS.NSIS --source winget"
  echo "或: winget install JRSoftware.InnoSetup --source winget"
fi

echo "完成。输出目录: $DIST"
