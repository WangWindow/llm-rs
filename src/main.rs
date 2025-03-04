/*
 * @FilePath: main.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-04 19:33:15
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-04 23:34:28
 * 2025 by WangWindow, All Rights Reserved.
 * @Description:
 */
use iced::{
    Font,
    widget::{Column, button, column, text},
    window::{Icon, Settings, icon},
};

fn main() -> iced::Result {
    iced::application("计数器", Counter::update, Counter::view)
        .font(include_bytes!("../res/fonts/SourceHanSansCN-Regular.otf")) // 载入字体文件
        .default_font(Font::with_name("思源黑体 CN")) // 设置默认字体
        .window(Settings {
            icon: Some(Icon::from(
                icon::from_file_data(include_bytes!("../res/icons/bingtang.ico"), None).unwrap(),
            )), // 设置窗口 icon
            ..Default::default() // 窗口的其余设置用默认
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
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("增加").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("减少").on_press(Message::Decrement),
        ]
    }
}

impl Counter {
    // ...

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
