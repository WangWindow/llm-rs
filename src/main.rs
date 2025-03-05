/*
 * @FilePath: main.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-04 19:33:15
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-05 22:47:14
 * 2025 by WangWindow, All Rights Reserved.
 * @Description: A simple LLM agent, using rig(API) and iced(UI).
 */

mod ui;

#[tokio::main]
async fn main() -> iced::Result {
    ui::run_app().await
}
