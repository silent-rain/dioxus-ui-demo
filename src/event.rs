//! 事件

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct ClickHandler(pub Arc<Mutex<dyn FnMut(Event<MouseData>) + 'static>>);

impl std::fmt::Debug for ClickHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Click Handler")
    }
}

// 实现 PartialEq trait
impl PartialEq for ClickHandler {
    fn eq(&self, other: &Self) -> bool {
        // 这里我们比较的是闭包的类型，而不是它们的行为
        // 如果你需要比较闭包的行为，那么这个实现将不适用
        std::ptr::eq(&*self.0, &*other.0)
    }
}

// 为 ClickHandler 实现 Deref trait
impl Deref for ClickHandler {
    type Target = Arc<Mutex<dyn FnMut(Event<MouseData>) + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
