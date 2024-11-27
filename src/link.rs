use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

use crate::core::{ComponentTrait, EleWarp};
use crate::event::ClickHandler;

/// Link 组件
/// 组件需要实现 Debug, Clone, Default, PartialEq trait
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Link {
    value: String,
    text: String,
    class: String,
    style: String,
    components: Vec<Element>,
    onclick: Option<ClickHandler>,
}

impl Link {
    pub fn new(text: &str) -> Self {
        Link {
            text: text.to_owned(),
            class: "w-24 h-8 leading-8 bg-blue-500 rounded-md shadow-md text-center".to_string(),
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

impl ComponentTrait for Link {
    /// 生成 HTML 元素
    fn build(self) -> Element {
        let onclick = self.onclick.clone();
        rsx! {
            a {
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
        Link::new("link")
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
