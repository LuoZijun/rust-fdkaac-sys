Rust FDK AAC bindings
=========================

:Date: 11/06 2018

.. contents::

前置条件
----------

请确保你的操作系统已经有了以下命令行程序可供使用(大部分系统里面都已经内置):

*   make
*   git
*   cc(gcc/clang)
*   c++(g++/clang++)
*   ar
*   nasm
*   autoreconf

*   mkdir
*   cp
*   bash



编译
-----------

.. code:: bash
    
    git clone --recurse-submodules -j8 https://github.com/LuoZijun/rust-fdkaac-sys.git

    cargo build


测试(Test)
-------------

.. code:: bash
    
    cargo test --release


用例
-------------

*Cargo.toml*:

.. code:: toml
    
    [dependencies]
    fdkaac-sys = { git = "https://github.com/LuoZijun/rust-fdkaac-sys" }
