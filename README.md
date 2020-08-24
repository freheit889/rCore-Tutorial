# rCore-Tutorial
rCore移植k210

### 目前已实现
* 在opensbi上运行lab1-6
* 在rustsbi上运行lab1-6
* 在sd卡上读写用户态

### 已支持的系统调用
* sys_open
* sys_exec

### 未实现
 * 更多用户态的支持
 * 多核支持


### 烧写用户态

  首先生成用户镜像,进入`downImg/rust/sdtest` 
  
  执行`make down` 进入烧写过程 
  
  等待提示完毕即可
  
  纯属娱乐 大家可以自己用dd命令
