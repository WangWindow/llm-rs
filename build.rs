/*
 * @FilePath: build.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-04 21:13:22
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-04 21:13:30
 * 2025 by WangWindow, All Rights Reserved.
 * @Description:
 */
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/ui/res/icons/bingtang.ico");
        res.compile().unwrap();
    }
}
