# until


```
重试命令直到满足条件，支持linux和windows以及macos
```


## 构建


使用docker实现musl静态编译和windows交叉编译

```shell
sh build.sh
```

输出在 `/out` 目录


## 用法

```shell
until --max 22 ping t.cn
```

max参数可选，默认为-1，代表无限重试






