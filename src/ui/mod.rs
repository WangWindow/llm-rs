/*
 * @FilePath: mod.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-05 22:33:40
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-05 22:48:20
 * 2025 by WangWindow, All Rights Reserved.
 * @Description: UI模块主文件
 */
pub mod components;
pub mod layout;

use components::Counter;
use iced::{
    Font,
    window::{Icon, Settings, icon},
};

/// 启动UI应用程序
pub async fn run_app() -> iced::Result {
    iced::application("LLM Agent", Counter::update, Counter::view)
        .font(include_bytes!("res/fonts/SourceHanSansCN-Regular.otf"))
        .default_font(Font::with_name("思源黑体 CN"))
        .window(Settings {
            icon: Some(Icon::from(
                icon::from_file_data(include_bytes!("res/icons/bingtang.ico"), None).unwrap(),
            )),
            ..Default::default()
        })
        .run()
}
