//! 核心库
use std::fmt::Debug;

use dioxus::prelude::*;

/// 组件声明
pub trait ComponentTrait
where
    Self: Debug + Clone + PartialEq + 'static,
{
    fn build(self) -> Element;
}

/// 组件包装
///
/// 组件需要实现 Debug, Clone, Default, PartialEq trait
#[component]
pub fn Wrap<C: ComponentTrait>(ele: C) -> Element {
    ele.build()
}

/// 组件元素包装
#[component]
pub fn EleWarp(ele: Element) -> Element {
    ele
}
