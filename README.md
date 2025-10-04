<div align="center">
    <img src="src-tauri/icons/icon.png" width="250" alt="logo">
</div>
<div align="center">

# LightMeetsPiano

</div>
轻量级光遇自动弹琴脚本，纯按键模拟，使用 Tauri + Vue3 + TypeScript + Rust 开发。

## 使用

支持直接导入 Sky Studio 导出的 TXT 谱，直接将其导入并将屏幕焦点切换到游戏窗口即可开始演奏。

## 下载

前往 [Releases](https://github.com/StillMisty/LightMeetsPiano/releases) 下载最新版本

windows 用户可下载 exe 或 msi 安装包，可直接点击授权运行
Linux 用户可下载 AppImage 或 deb 包安装，运行请使用 sudo 权限运行

Arch 用户可通过 AUR 直接安装

```bash
paru -S lightmeetspiano-bin
```

## 开发

```bash
git clone https://github.com/StillMisty/LightMeetsPiano
cd LightMeetsPiano
# 安装依赖
pnpm i
# 启动
sudo pnpm tauri dev
# 打包
sudo pnpm tauri build
```

## 注意

- 因为涉及到按键模拟，所以不管是Windows还是Linux都需要以管理员权限运行，介意勿用
- 仅支持在Windows环境下播放音乐自动激活游戏窗口，Linux环境下需要手动切换到游戏窗口
- TXT 谱仅支持 Sky Studio 导出的未加密格式，其他格式请自行转换
- 为避免版权争议，该项目不提供任何谱子，请自行寻找