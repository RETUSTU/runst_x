//! 系统常量定义
//! 
//! 包含系统版本号、编译信息、显示设置等常量

/// 系统名称
pub const SYSTEM_NAME: &str = "RunST X";

/// 系统版本号
pub const SYSTEM_VERSION: &str = "25.12.13C";

/// 系统完整名称
pub const SYSTEM_FULL_NAME: &str = "RunST X v25.12.13C";

/// 系统描述
pub const SYSTEM_DESCRIPTION: &str = "UEFI Operating System";

// 包含自动生成的编译信息
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

/// 系统提示符
pub const SYSTEM_PROMPT: &str = "RunST X> ";

/// 欢迎消息
pub const WELCOME_MESSAGE: &str = "System initialized successfully!";

/// 官网链接
pub const WEBSITE_URL: &str = "https://runst.rtstudio.top";

/// 帮助提示信息
pub const HELP_PROMPT: &str = "Type 'help' to list commands.";

/// 未知命令提示信息
pub const UNKNOWN_COMMAND_HELP: &str = "Type 'help' for list.";

/// 默认内存大小 (MB)
pub const DEFAULT_MEMORY_SIZE: u64 = 512;

/// 默认屏幕宽度
pub const DEFAULT_SCREEN_WIDTH: u32 = 80;

/// 默认屏幕高度
pub const DEFAULT_SCREEN_HEIGHT: u32 = 25;

/// 支持的命令列表
pub mod commands {
    pub const VER: &str = "ver";
    pub const HELP: &str = "help";
    pub const REBOOT: &str = "reboot";
    pub const SHUTDOWN: &str = "shutdown";
    pub const ECHO: &str = "echo";
    pub const CLEAR: &str = "clear";
}

/// 命令描述
pub mod command_descriptions {
    pub const VER: &str = "Show system version";
    pub const HELP: &str = "Show this help";
    pub const REBOOT: &str = "Reboot the system";
    pub const SHUTDOWN: &str = "Shutdown the system";
    pub const ECHO: &str = "Echo text to screen";
    pub const CLEAR: &str = "Clear the screen";
}

/// 错误消息
pub mod error_messages {
    pub const UNKNOWN_COMMAND: &str = "Unknown command: {}";
    pub const COMMAND_NOT_FOUND: &str = "Command not found";
    pub const INVALID_ARGUMENT: &str = "Invalid argument";
    pub const SYSTEM_ERROR: &str = "System error";
}

/// 成功消息
pub mod success_messages {
    pub const SYSTEM_INITIALIZED: &str = "System initialized successfully!";
    pub const COMMAND_EXECUTED: &str = "Command executed successfully";
}

/// 警告消息
pub mod warning_messages {
    pub const LOW_MEMORY: &str = "Low memory warning";
    pub const SYSTEM_BUSY: &str = "System is busy";
}

/// 信息消息
pub mod info_messages {
    pub const SYSTEM_INFO: &str = "System information";
    pub const COMMAND_INFO: &str = "Command information";
}