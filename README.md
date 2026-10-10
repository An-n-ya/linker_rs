# linker_rs

一个用 Rust 写的 ELF 链接器，写着玩 + 学习链接器原理，整体思路参考了 [mold](https://github.com/rui314/mold) 作者的 [Linker 设计博客](https://linkerbook.wordpress.com/)。

## 目前做了什么

- 解析 gcc 传给 ld 的命令行参数（配合 `gcc -B.` 直接把本链接器当 ld 用）
- ELF 文件头、section、symbol 的读取
- 静态库（.a）解析，支持 `-l` 和库搜索路径
- 符号解析，未用到的目标文件会被剔除
- 可合并 section（`SHF_MERGE` / `SHF_STRINGS`）的处理，还在进行中

## 使用

```bash
make build   # 编译并创建 ld 符号链接
make test    # 跑 tests/ 下的所有测试
```

测试脚本用 `gcc -B.` 让 gcc 调用本链接器完成链接，输出到 `out/tests/`。

## 目录结构

```
src/
├── archive_parser.rs    # .a 静态库解析
├── argument_parser.rs   # 命令行参数
├── context.rs           # 全局上下文
├── e_header/            # ELF header
├── linker.rs            # 主流程
├── output_section/      # 输出 section
├── passes.rs            # 链接各阶段
├── section.rs           # section 表示
└── symbol.rs            # 符号
```
