# BLOG OS

**tip: use nightly rust and dont rely any OS**

***it is study by `https://os.phil-opp.com/`***

- xbuild 工具: 这个工具封装了 cargo build；但不同的是，它将自动交叉编译 core 库和一些编译器内建库（compiler built-in libraries）
- xrun: 和 xbuild 类似，xrun 子命令将在调用 cargo 命令前编译内核所需的包。这个子命令也由 cargo-xbuild 工具提供，所以你不需要安装额外的工具
- xtest: 也是由 cargo-xbuild 工具提供