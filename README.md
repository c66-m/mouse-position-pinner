<div align="center">

# 鼠标位置定位器

**实时显示鼠标全局坐标的置顶小窗，按 F2 一键复制到剪贴板。**

![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5-3178C6?logo=typescript&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-52b345)
![Version](https://img.shields.io/badge/version-0.1.0-orange)

</div>

---

一个约 **190×64** 的无边框、透明、置顶小窗，常驻屏幕角落，以约 30ms 的频率实时显示鼠标全局坐标。按 **F2** 把当前坐标以 `1234, 567` 的形式写入剪贴板，粘贴即用。基于 Tauri 2 构建，安装包轻量（约 8 MB），内存占用极低，不干扰正常操作。

> 原名 CoordPin，现更名为「鼠标位置定位器」。

## 快速开始

| 你想… | 怎么做 |
| --- | --- |
| 看鼠标当前在哪 | 小窗常显 `X: 1234  Y: 567`，跟随鼠标实时刷新 |
| 复制当前坐标 | 按 **F2**（可在设置里改成任意热键），坐标即写入剪贴板 |
| 拖动小窗到别处 | 直接按住小窗拖，位置下次打开自动恢复 |
| 修改热键 / 关闭置顶 | 鼠标悬停小窗 → 右上角点 **⚙** → 改完点「完成」 |
| 退出 | 悬停小窗 → 右上角点 **×** |

## 功能

| 功能 | 说明 |
| --- | --- |
| 实时坐标 | 窗口常显 `X: 1234  Y: 567`，约 30ms 刷新一次；跨多显示器使用**虚拟屏坐标**（主屏左上角为 (0,0)，左侧/上方副屏坐标为负值） |
| 热键复制 | 默认 **F2**（可在设置中修改），按下即把当前坐标以纯文本 `1234, 567` 写入剪贴板，窗口短暂显示「已复制 ✓」 |
| 可拖拽 | 按住窗口任意位置拖动；位置自动记忆，下次打开还在原处 |
| 小控制点 | 鼠标悬停时右上角露出 ⚙ 设置、× 退出两个小按钮 |
| 设置持久化 | 热键、置顶开关、窗口位置写入 `%APPDATA%\MousePositionPinner\settings.json`（Windows；macOS/Linux 见下方路径） |

明确不做：颜色取色、放大镜、开机自启、托盘图标、多副本。

## 为什么需要它

在做 UI 布局、游戏脚本、自动化测试、设计稿还原时，经常需要反复确认「鼠标现在停在哪个像素」。系统自带的取色器不常驻、截图工具要手动触发、在线坐标测试仪要切浏览器——这个小窗就是常驻在屏幕角落的「坐标读数器」，按一下热键就把坐标塞进剪贴板，直接粘到代码、文档或对话框里。

## 技术要点

- **前端**：Vite + React + TS；夜间深色小面板。坐标轮询走 `invoke('get_cursor_pos')`；`copy-done` 事件驱动「已复制 ✓」反馈；设置弹层打开时窗口自动展开为 230×150，关闭恢复 190×64。
- **后端**：Rust 命令 `get_cursor_pos`（`GetCursorPos` 虚拟屏坐标）+ `tauri-plugin-global-shortcut`（可注册/注销/改热键）+ `tauri-plugin-clipboard-manager`（剪贴板）+ JSON 设置读写（写临时文件 + 原子重命名，防止崩溃损坏配置）。
- **窗口**：无边框、透明背景、圆角 10px、置顶、跳过任务栏、不可缩放；拖拽用 CSS `app-region: drag`，交互元素 `no-drag`。
- **热键语法**：`F2`、`Ctrl+Shift+A`、`Win+Alt+9`、`F13` 等；支持修饰键 Ctrl / Alt / Shift / Win(Cmd)，大小写与空格不敏感。注册失败（被占用）会给出明确错误。

## 项目结构

```
鼠标位置定位器/
├── index.html / package.json / vite.config.ts / tsconfig.json
├── src/                          # React 前端
│   ├── App.tsx                   # 主组件：坐标展示、悬停控制、复制反馈
│   ├── App.css                   # 夜间深色主题
│   ├── hooks/useCursorPosition.ts# 30ms 轮询光标坐标
│   ├── components/SettingsPopup.tsx
│   ├── lib/coords.ts             # 坐标格式化（前后端口径一致）
│   └── *.test.tsx / *.test.ts    # vitest 测试
├── src-tauri/                    # Rust 后端
│   ├── tauri.conf.json           # 窗口 / 打包 / NSIS 配置
│   ├── capabilities/default.json # IPC 权限
│   ├── icons/                    # 各平台图标（含 icon.ico / icon.icns）
│   └── src/
│       ├── main.rs / lib.rs      # 装配：命令、热键、剪贴板、位置记忆
│       ├── cursor.rs             # GetCursorPos 虚拟屏坐标
│       ├── hotkey.rs             # 热键字符串解析 + 校验
│       └── settings.rs           # settings.json 读写（原子写入）
└── scripts/gen_icon.py           # 图标生成脚本
```

## 开发与构建

环境要求：Node 18+、Rust stable、以及 Tauri 的 [Linux 系统依赖](https://v2.tauri.app/start/prerequisites/)（Windows/macOS 无需额外依赖）。

```bash
npm install                 # 安装前端依赖
npm run tauri dev           # 开发模式（热更新）

npm test                    # 前端单元测试（vitest）
npm run build               # 前端类型检查 + 构建（tsc + vite）
cd src-tauri && cargo test  # Rust 单元测试

npm run tauri build         # 打包安装包
```

### 安装包产物

- **Windows**：`src-tauri/target/release/bundle/nsis/鼠标位置定位器_0.1.0-setup.exe`（NSIS 安装器，含简体中文/英文语言）
- **Linux**：`src-tauri/target/release/bundle/deb/鼠标位置定位器_0.1.0_amd64.deb`

> 注：Windows NSIS 安装包需在 Windows 上执行 `npm run tauri build`（Tauri 不支持跨系统交叉打包 Windows 安装器）；源码与配置已就绪，在 Windows 上装好 Node + Rust 后一条命令即可出包。

## 配置文件

`settings.json` 示例（Windows 路径 `%APPDATA%\MousePositionPinner\settings.json`；macOS `~/Library/Application Support/MousePositionPinner/settings.json`；Linux `~/.config/MousePositionPinner/settings.json`）：

```json
{
  "hotkey": "F2",
  "alwaysOnTop": true,
  "window": { "x": 1650, "y": 40 }
}
```

`window` 字段由程序在拖动时自动写入；恢复位置时若窗口已不在任何显示器范围内（例如副屏被拔掉），会自动回退到主屏右上角。

## 测试覆盖

- **Rust 单元测试**（`cargo test`）：热键解析（大小写/空白/非法输入/修饰键组合/F1–F24）、设置读写（默认值、JSON 回读、缺失/损坏文件回退、camelCase 字段解析）、坐标文本格式化（正/负/零）。
- **前端测试**（`npm test`）：坐标格式化工具、组件渲染（坐标来自 `get_cursor_pos`）、`copy-done` 事件触发「已复制 ✓」、设置弹层读写热键、退出按钮。

---

<div align="center">

**License**：MIT（自行选用，无限制）  
**Author**：c66-m

</div>
