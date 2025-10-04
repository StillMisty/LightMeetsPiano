<div align="center">
    <img src="src-tauri/icons/icon.png" width="250" alt="logo">
</div>
<div align="center">

# LightMeetsPiano

</div>
轻量级光遇自动弹琴脚本，纯按键模拟，使用 Tauri + Vue3 + TypeScript + Rust 开发。

## 使用方法

支持直接导入 Sky Studio 导出的 TXT 谱，直接将其导入并将屏幕焦点切换到游戏窗口即可开始演奏。

## 注意

- 因为涉及到按键模拟，所以不管是Windows还是Linux都需要以管理员权限运行，介意勿用
- 仅支持在Windows环境下播放音乐自动激活游戏窗口，Linux环境下需要手动切换到游戏窗口
- TXT 谱仅支持 Sky Studio 导出的未加密格式，其他格式请自行转换
- 为避免版权争议，该项目不提供任何谱子，请自行寻找
