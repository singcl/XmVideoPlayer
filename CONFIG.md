git config 在线文档

https://git-scm.com/docs/git-config#_configuration_file

> 文档-https://git-scm.com/docs/git-config#_configuration_file中描述了该格式。它没有特定的“标准”格式，只是git所期望的格式。
> 它类似于 ini 文件，但不完全相同—有带引号的子部分和值。

``git config --help`
打开本地 git config 帮助文档

```sh
# 查看所有配置以及配置所在文件
git config --list --show-origin
# 查看所有配置以及配置的范围
git config --list --show-scope
# 查看global 配置
git config --global --list
```

设置代理和删除代理

```sh
# 设置代理
git config --global http.proxy 127.0.0.1:55443
git config --global https.proxy 127.0.0.1:55443
# 删除代理
git config --global --unset http.proxy
git config --global --unset https.proxy
```

npm 临时

```sh
npm --registry=https://registry.npm.taobao.org install ***
npm install --registry=https://registry.npm.taobao.org

```

npm 永久

```sh
npm config set registry https://registry.npm.taobao.org
# 配置后可通过下面方式来验证是否成功
npm config get registry 或 npm info ***
```

在根目录下编写~/.npmrc 文件

```sh
registry=https://registry.npm.taobao.org
NODE_OPTIONS=--max-old-space-size=4096
```

使用 nrm 管理 npm 的镜像

```
npm i -g nrm
nrm ls
```
