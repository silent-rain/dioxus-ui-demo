use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

use crate::core::{ComponentTrait, EleWarp};
use crate::event::ClickHandler;

/// Card 组件
/// 组件需要实现 Debug, Clone, Default, PartialEq trait
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Card {
    value: String,
    text: String,
    class: String,
    style: String,
    components: Vec<Element>,
    onclick: Option<ClickHandler>,
}

impl Card {
    pub fn new() -> Self {
        Card {
            class: "w-[478px] h-[196px] bg-white rounded-md shadow-md".to_string(),
            onclick: None,
            ..Default::default()
        }
    }

    /// 子组件
    pub fn components(mut self, components: Vec<Element>) -> Self {
        self.components = components;
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class.push_str(class);
        self
    }
    pub fn style(mut self, style: &str) -> Self {
        self.style = style.to_owned();
        self
    }
    pub fn width(mut self, width: usize) -> Self {
        if !self.style.ends_with(";") {
            self.style.push(';');
        }
        self.style.push_str(&format!("width: {width}px"));
        self
    }
    pub fn height(mut self, height: usize) -> Self {
        if !self.style.ends_with(";") {
            self.style.push(';');
        }
        self.style.push_str(&format!("height: {height}px"));
        self
    }

    /// 点击事件
    pub fn onclick<F>(mut self, onclick: F) -> Self
    where
        F: FnMut(Event<MouseData>) + 'static,
    {
        self.onclick = Some(ClickHandler(Arc::new(Mutex::new(onclick))));
        self
    }
}

impl ComponentTrait for Card {
    /// 生成 HTML 元素
    fn build(self) -> Element {
        let onclick = self.onclick.clone();
        rsx! {
            div {
                class: "{self.class}",
                style: "{self.style}",
                onclick: move |event| {
                   let onclick= match  &onclick {
                        Some(onclick) => onclick,
                        None => return,
                    };
                    let mut onclick = onclick.lock().unwrap(); // 获取锁
                    (onclick)(event); // 调用闭包
                },
                for ele in self.components {
                    EleWarp { ele: ele }
                },
                "{self.text}",
             }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        Card::new()
            .class("down-low")
            .width(96)
            .height(32)
            .style("color: #fff;")
            .onclick(|_| {
                println!("onclick");
            })
            .build();
    }
}
