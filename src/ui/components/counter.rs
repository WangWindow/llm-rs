/*
 * @FilePath: counter.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-05 22:33:40
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-05 22:41:23
 * 2025 by WangWindow, All Rights Reserved.
 * @Description: 计数器组件
 */

use iced::widget::{Column, button, column, text};

/// 计数器应用的状态
#[derive(Default)]
pub struct Counter {
    value: i32,
}

/// 计数器相关的消息类型
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    /// 创建计数器视图
    pub fn view(&self) -> Column<Message> {
        column![
            button("增加").on_press(Message::Increment),
            text(self.value).size(50),
            button("减少").on_press(Message::Decrement),
        ]
    }

    /// 处理计数器消息
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
