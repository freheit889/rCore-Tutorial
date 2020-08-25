# rCore-Tutorial
rCore移植k210

## 目前已实现
* 在opensbi上运行lab1-6
* 在rustsbi上运行lab1-6
* 在sd卡上读写用户态
* 虚拟存储

### 已支持的系统调用
* sys_open
* sys_exec

### 未实现
 * busyBox
 * 更多用户态的支持
 * 多核支持



## 环境配置
#### 安装rust

```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

进入rCore文件夹下 执行 
```
rustup target add riscv64imac-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
等待cargo将所有依赖包装完之后 环境配置就完成了

### 烧写用户态

  dd烧写镜像：https://www.shangmayuan.com/a/054a78da4a6e4bc589bd4fad.html
  
  我们的镜像是 user/build/raw.img


### 执行程序

  在 os目录下 执行make run-k210 
  
  因为使用的是SD卡  所以程序执行的过程可能会慢一些
  
  如果不使用较大的内存 可以在/src/progress/config.rs中调节线程运行栈的大小   可以有效提高运行速度
