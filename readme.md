# hiss

基于rust+axum+seaorm+postgresql的短链接 SaaS 服务

## 开发：
安装 cargo-watch `cargo install cargo-watch`  
执行 `cargo watch -x run`

## todo：
- [x] 自定义axum的Json extractor 
- [x] 规范内部请求错误格式 
- [x] 规范内部请求成功格式
- []  访问短链时，数据库记录访问者信息改为异步处理
- []  请求限速器中间件
- []  接入swagger
- []  接入jwt鉴权
- []  完善saas结构
- []  完善RBAC模型


## 目录结构介绍：
``` lua
|   config.rs            解析.env的配置文件，声明.env里的类型，方便获取.env的各种数据
|   err.rs               公共的错误处理包，捕捉和处理sea-orm和app等全局的错误
|   main.rs              程序入口
|   param.rs             请求参数的结构声明与校验等
|   router.rs            路由
|   state.rs             全局的状态库，挂载数据库到app上
|   tool.rs              工具库
|
\---entity               数据库结构
    |   mod.rs           类似于 js 里面的 index.js
    |   ...
\---handler              处理函数
    |   mod.rs           类似于 js 里面的 index.js
    |   ...
```


## DEMO创建短链：
``` bash
curl --location 'http://pkjy.xyz/short_url' \
--header 'Content-Type: application/json' \
--data '{"short_domain":"pkjy.xyz","original_url":"https://www.baidu.com"}'
```
修改`original_url`为目标url。


## 性能简单测试：
记录访问ua同步插入时：
``` bash
mac@PKJY-MacPro ~ % wrk http://127.0.0.1:9527/diqIMh -c200 -t10 -d30s
Running 30s test @ http://127.0.0.1:9527/diqIMh
  10 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    78.97ms   53.55ms 699.72ms   95.08%
    Req/Sec   273.02     88.45   490.00     68.26%
  80721 requests in 30.08s, 9.39MB read
  Socket errors: connect 0, read 88, write 5, timeout 0
Requests/sec:   2683.75
Transfer/sec:    319.74KB
```
