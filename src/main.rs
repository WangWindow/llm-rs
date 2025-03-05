/*
 * @FilePath: main.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-04 19:33:15
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-05 19:55:30
 * 2025 by WangWindow, All Rights Reserved.
 * @Description: A simple LLM agent, using rig(API) and iced(UI).
 */
use iced::{
    Font,
    widget::{Column, button, column, text},
    window::{Icon, Settings, icon},
};
use rig::{agent::AgentBuilder, completion::Prompt, providers::deepseek};

#[tokio::main]
async fn main() -> iced::Result {
    // 创建 LLM 客户端
    let client = deepseek::Client::from_env();
    let deepseek = client.completion_model(deepseek::DEEPSEEK_CHAT);
    let agent = AgentBuilder::new(deepseek)
        .preamble(
            "\
        You are Gandalf the white and you will be conversing with other \
        powerful beings to discuss the fate of Middle Earth.\
    ",
        )
        .build();
    let response = agent
        .prompt("What is the best way to defeat Sauron?")
        .await
        .unwrap();
    println!("{response}");

    // 创建应用程序
    iced::application("LLM Agent", Counter::update, Counter::view)
        .font(include_bytes!("../res/fonts/SourceHanSansCN-Regular.otf"))
        .default_font(Font::with_name("思源黑体 CN"))
        .window(Settings {
            icon: Some(Icon::from(
                icon::from_file_data(include_bytes!("../res/icons/bingtang.ico"), None).unwrap(),
            )),
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        column![
            button("增加").on_press(Message::Increment),
            text(self.value).size(50),
            button("减少").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
