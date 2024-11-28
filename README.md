# Dioxus UI

注意，这目前仅是一个可行性调研的Demo。

## 介绍

Dioxus UI 是一个基于 Rust 语言的声明式 UI 开发框架，专为构建现代跨端应用界面而设计。Dioxus UI 在 Dioxus 框架的基础上开发，结合了 Rust 的性能优势和声明式编程的便利性，为开发者提供了一个强大且灵活的工具来创建高效、可靠的用户界面。

### 主要特性

- 声明式编程：
采用声明式的编程范式，使开发者能够以更直观和高效的方式定义用户界面。通过描述 UI 的结构，而不是直接操作 DOM，简化了开发过程并减少了代码量。

- 丰富的 UI 组件：
内置了一系列常用的 UI 组件，如按钮、输入框、列表等，每个组件都经过精心设计，易于使用且功能强大。开发者可以根据需要自由组合这些组件，快速搭建复杂的用户界面。

- 实时界面预览：
提供强大的实时预览功能，允许开发者在编写代码的同时即时查看界面效果。这不仅加快了开发速度，还使得调试变得更加方便。

- 跨平台支持：
使用 Dioxus，您可以在多种设备和浏览器上提供一致的用户体验。无论是桌面端还是移动端，都能轻松应对，确保您的应用在不同环境下都能表现出色。

- 高效的性能：
得益于 Rust 语言的高性能特性，Dioxus 框架能够在保证功能丰富的同时，依然保持出色的运行效率。这对于需要处理大量数据或复杂交互的应用尤为重要。

## 开发

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
dx serve  --port 8187 --open --hot-reload
```

- Open the browser to <http://localhost:8080>
