# brainfuck-interpreter

## 参赛信息

- 项目：<https://github.com/StarryReverie/brainfuck-interpreter.git>

## 基本介绍

本项目实现了一个通用的 brainfuck 语言的解释器和 REPL，通过读取 brainfuck 源代码来解释运行。本项目的关键优势在于解释器不是一个朴素的实现，而是包括了各种优化的高效实现，并且在运行的配置上非常灵活，支持自定义各种选项。关键功能特性包括：

- 专用的 IR 设计，实现了词法分析、语法分析、优化、IR 生成、执行的流水线，优化执行性能。
- Multi-pass 优化器，包括多种优化策略。
- 可配置的运行时参数，包括内存单元大小、内存单元类型、输入输出流类型等。
- 详细的命令帮助，使用 `bf-interpreter --help` 查看各种帮助信息。

除此之外，项目还有一些隐含的优势，包括：

- 清晰的模块划分和复用，语言核心实现可复用，支持解释器和 REPL 两套操作接口。
- 测试覆盖高，对于关键的核心实现编写了完备的测试，对解释器外围接口进行集成测试。
- 包括 benchmark 代码，对解释器进行性能分析。

## 环境说明

本项目为跨平台应用，支持任意操作系统，支持一份代码、随处构建。

运行以下命令构建所有应用程序：

```bash
cargo build --release
```

运行以下命令解释执行 brainfuck 程序：

```bash
cargo run --release --bin bf-interpreter -- </path/to/source/file> # 后面可以添加任意参数和选项
```

运行以下命令安装应用到本地：

```bash
cargo install --path ./crates/bf-interpreter
```

详细的使用方法参考 [README.md](./README.md)。

## 生成式 AI 使用说明

本项目没有使用任何生成式 AI 功能。

## 许可证

本项目以 MIT 许可证发布.

Copyright (C) 2023-2026 Justin Chen (StarryReverie)
