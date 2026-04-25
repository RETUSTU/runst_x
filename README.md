# RunST X

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![UEFI](https://img.shields.io/badge/UEFI-x86__64-green.svg)](https://uefi.org/)

**RunST X** 是一个使用 Rust 语言开发的 UEFI 64 位 DOS-Like 操作系统。它将经典的 DOS 命令行交互体验与现代 UEFI 引导技术相结合，为技术爱好者和底层系统开发者提供了一个简洁、可研究的操作系统示例。

> **项目状态**：RunST X 为旧有项目，现已停止常规更新，但正式开源以供学习与参考。

🌐 **项目官网**：[runst.rtstudio.top](https://runst.rtstudio.top)  
🏢 **工作室官网**：[rtstu.com](https://rtstu.com) | 旧域名：[rtstudio.top](https://rtstudio.top)

---

## 📖 项目简介

RunST X 由热土工作室（RTStudio）开发，旨在探索 Rust 语言在 UEFI 环境下的系统编程能力，同时复现 DOS 时代直接与硬件对话的简洁体验。系统完全在 UEFI 64 位环境下运行，无需依赖传统的 BIOS 引导方式，兼容现代计算机硬件。[reference:0]

项目诞生于对底层技术的好奇与热情，所有代码现已在 Apache 2.0 许可下开源，希望能为对操作系统开发、Rust 底层编程或 UEFI 应用开发感兴趣的开发者提供一份有价值的参考。

---

## ✨ 系统特性

| 特性 | 说明 |
|------|------|
| **UEFI 64 位引导** | 完全支持现代 UEFI 64 位引导标准，兼容最新的 x86_64 计算机硬件，告别传统 BIOS 的限制。[reference:1] |
| **模块化设计** | `bootx64.efi` 引导程序与 `kernel.nep` 内核分离架构，使系统易于维护、更新和扩展。[reference:2] |
| **色彩显示支持** | 支持丰富的色彩显示，为经典的命令行界面带来现代化的视觉体验。[reference:3] |
| **轻量高效** | 系统体积小巧（核心文件约 70KB），启动迅速，资源占用极少，适合嵌入式系统或学习研究使用。[reference:4] |

---

## 🏗️ 系统架构

RunST X 采用简洁的两阶段引导设计：
┌─────────────────┐ ┌─────────────────┐
│ bootx64.efi │ ──▶ │ kernel.nep │
│ (引导程序) │ │ (系统内核) │
│ ~33KB │ │ ~35KB │
└─────────────────┘ └─────────────────┘

- **bootx64.efi**：UEFI 应用程序，负责初始化硬件环境、加载内核并移交控制权。
- **kernel.nep**：系统内核，实现命令行交互、文件系统操作等核心功能。

此架构使得引导逻辑与核心功能分离，便于开发者独立修改和升级各个组件。[reference:5]

---

## 🚀 快速开始

### 系统要求

- 支持 UEFI 64 位的 x86_64 计算机（或虚拟机/模拟器）
- 支持 UEFI 引导的介质（U 盘、虚拟硬盘等）

### 运行方式

1. **下载镜像**：从 [123 云盘](https://www.123865.com/s/NzUYjv-N5Utd?pwd=rtio#) 下载虚拟硬盘文件（约 6MB）[reference:6]
2. **启动系统**：使用虚拟机软件（如 QEMU、VirtualBox）或 Ventoy 等工具加载镜像并启动。
3. **进入系统**：在引导菜单中选择对应选项（输入 `1` 后回车）进入 RunST X 系统。[reference:7]
4. **开始体验**：在 `C:\>` 提示符下输入命令，如 `dir` 查看文件、`runstx --version` 查看版本等。[reference:8]

> 💡 **提示**：你也可以通过编译源码自行生成引导镜像。

