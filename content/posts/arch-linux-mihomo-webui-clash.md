---
id: 24
title: "Arch Linux 代理配置: 裸核 mihomo + WebUI (Clash系)"
slug: "arch-linux-mihomo-webui-clash"
summary: "在 Arch Linux 上把代理从 1GB 的图形客户端换成 60MB 的裸核 mihomo + WebUI 的完整指南。文章从 GFW 的四层拦截机制讲起，让读者先理解代理到底在解决什么问题；随后手把手走完整个配置流程——国内镜像源解决\"没代理怎么装代理\"、geo 数据文件的离线获取、systemd 用户服务常驻、TUN 全局接管与 setcap 授权、caddy 反代实现免端口访问、以及 metacubexd 面板的跨域配置。同时覆盖从 mihomo-party 迁移的完整路径与排错速查。面向刚装好 Arch 的新手和想精简系统的迁移者，全程 20 分钟可跑通。"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/arch-linux-mihomo-webui-clash.jpg"
status: "published"
is_featured: true
theme: "Linux系统 (linux)"
tags: ["教程", "archlinux", "linux", "network"]
view_count: 52
created_at: "2026-08-11 00:03:21"
updated_at: "2026-08-17 15:08:14"
source: "https://blog.yeastar.xin/posts/arch-linux-mihomo-webui-clash"
---

## 为什么要写这篇

一年前我在 Arch 上装 `mihomo-party` 包管理代理, 他是 Electron 套壳, 界面漂亮, 功能齐全

代价是 **光图形壳就吃 1GB 内存**, 真正起代理作用的 mihomo 核心只有 61MB

后来我看内存占用不顺眼, 就把他拆了: 裸核 + systemd 服务 + WebUI, 核心内存占用不超 64MB

这篇博客写给两类人:

* **迁移者:** 正在使用 `mihomo-party` / `Clash Verge` 这类图形客户端, 嫌重, 想换裸核 + WebUI (推荐重点看 `0x09` 章节)
* **新手:** 刚装好 Arch, 急需代理上外网, 连接 AUR, 但对代理还一头雾水的杂鱼

> 其实我在写 [archlinux 安装](https://blog.yeastar.xin/posts/archlinux-guide) 就想着要写这一篇了)
> 但谁叫我是个大懒鬼呢~ 而且我觉得自己不太适合教人(因为我总喜欢把知识强行灌给读者)
> 不过嘛, 既然你看到这篇博客, 就说明我又要开始灌知识了

---

## 0x00 为什么需要代理

> 我想从 GFW 的视角来谈这件事, 这也是国内用户最常用到的, 需要 "翻墙" 的情况

代理的本质是"绕路", 你到达不了某个服务器, 就让一台能到的服务器替你去

那为什么你会到不了呢, 这就是 **GFW(防火长城)** 在做的事情了, 他是一套部署在网络骨干上的过滤系统, 拦截手段大概分为四层:

#### 1) DNS 污染

> 如果你要访问 `google.com`, 第一步一定是查 DNS(域名转IP), GFW 可以在这一步假装 DNS 服务器, 抢答一个错误的IP地址给你

具体表现为: 域名解析看到一堆乱IP, 或者解析超时...

对抗手段: 不再本地解析敏感域名, 让代理服务器在境外解析 (DoH/DoT), 或者用 mihomo 的 fake-ip 模式, 本地分配假IP, 真实解析在远端

#### 2) IP 封锁

> 直接讲某些 IP 端判定为不可达, 让你无法完成 TCP 握手, 那自然就出不去了

对抗手段: 流量先到境外的代理服务器, 再由代理服务器去访问目标, 因为 GFW 几乎不拦截你访问"一台看起来正常的境外服务器"

#### 3) 端口封锁

> 即使 IP 联通, 历史上也有针对某些 IP 的端口封锁例子

对抗方式: 代理流量伪装为 HTTPS, 走常见端口, 例如全世界都在用的80, 443, GFW是不可能连这个都断的

#### 4) 连接重置与主动探测

> GFW 检查 TLS 握手, 如果发现 SNI 字段 (这是 TLS 中明文传输的域名) 是敏感字段, 就直接像通信双方发送 RST 包, 强制断开连接

具体表现为: TCP 能连上, 但一握手就断, 反复尝试反复断

> 最阴的来了: GFW 会主动向疑似代理服务器的目标发送探测包, 如果他的行为像代理, 就直接封锁

对抗手段: 普通用户到了这一层几乎就没有手段了, TLS 指纹模仿, 流量特征隐藏, 动态端口技术, 协议伪装 ... 本质上都是代理商在负重前行

GFW 的检测能力一直在升级, 而好的机场也会不断更新协议和伪装方式, 而没技术力的机场, 往往就很容易被斩杀

> mihomo 就是帮你绕过 GFW 的程序, 他读取 `config.yaml` 中的代理节点, 协议, 规则等信息, 将一个个数据包伪装起来, 安全的通过 GFW 这座大山

---

## 0x01 代理的三层结构

所有 Clash 系代理工具都可以看作这么三层:

```text
核心(mihomo)        >   连接代理服务器, 转发流量, 走规则
配置文件(yaml)      >   代理节点, 分组, 规则
管理端(GUI/WebUI)   >   在图形化页面切节点, 看延迟, 查连接
```

`mihomo-party` 把这三层结构打包在一起, 还套了一个 Electron 壳, 裸核方案就是把壳拆掉:

```text
mihomo 核心 ──► systemd 用户服务常驻
   ├── 7890 混合端口（代理入口）
   ├── 9090 API（管理接口）
   └── TUN 虚拟网卡（全局接管）
metacubexd 面板  ◄── 浏览器打开，纯静态页
```

核心与面板之间通过 9090 端口的 HTTP API 通信, 只要理解了这个, 后面的操作就很简单啦~

## 0x02 准备 `config.yaml`

> 经典的 **先有鸡还是先有蛋** 问题

mihomo 要求一份配置文件, 里面是机场给你的节点, 这个东西请自行获取啦~

* 有些机场可能会提供 "Clash / Mihomo 订阅链接", 复制链接然后使用 `curl -o config.yaml '订阅连接'` 就可以下载
* 有些机场可能会给 v2rayN 格式, 需要通过订阅转换工具 (例如 `subconverter`) 转换成 Clash 格式使用

查看配置文件, 只要你看到了 `proxies`, `proxy-groups`, `rules` 这三段, 那就下对啦

同时记得检查一下 `external-controller`, 某些订阅文件可能没有这一项, 裸核要靠 9090 API 跟面板通信:

```yaml
external-controller: 127.0.0.1:9090
```

将配置文件放进配置目录:

```bash
mkdir -p ~/.config/mihomo 
cp config.yaml ~/.config/mihomo/config.yaml 
```

> 如果你刚刚装好 Arch / 还没有代理, 下载不到也是正常的捏
> 这是一个很经典的 Arch 问题: 我要出网下很多东西, 所以需要代理, 但是下载代理所需的包需要出网

---

## 0x03 配置 archlinuxcn 国内源

> `mihomo` 本体和面板都在 archlinuxcn 仓库, 这个仓库在国内有镜像, 不需要代理就能装, 这样就解决了 "没代理怎么装代理" 的问题

编辑 `/etc/pacman.conf`, 在末尾添加:

```ini
[archlinuxcn]
Server = https://mirrors.tuna.tsinghua.edu.cn/archlinuxcn/$arch 
```

> 这个是清华源, 如果速度比较慢的话, 可以去网上搜搜阿里源, 中科大源什么的

```bash
sudo pacman -Syy
sudo pacman -S archlinuxcn-keyring  # 信任仓库密钥
sudo pacman -S mihomo metacubexd-bin
```

万一 `archlinuxcn-keyring` 装不上, 那可以试试手动安装:

```bash
wget https://mirrors.tuna.tsinghua.edu.cn/archlinuxcn/archlinuxcn-keyring.pkg.tar.zst
sudo pacman -U archlinuxcn-keyring.pkg.tar.zst
```

装完验证一下: `pacman -Q mihomo`

顺便看看面板文件在不在:

```bash
ls /usr/share/metacubexd | head
```

---

## 0x04 下载 geo 数据文件

> mihomo 的规则里有 `GEOIP,CN` 这类国家判断, 需要 geo 数据文件, 他默认在你启动 `mihomo` 的时候联网下载, 但此时还没有代理, 所以下载必超时, 然后启动失败

从 pacman 仓库下载

```bash
sudo pacman -S clash-geoip v2ray-rules-dat

# MMDB 格式（geoip 规则用）
sudo cp /etc/clash/Country.mmdb ~/.config/mihomo/geoip.metadb 

# geosite 格式（域名规则用），路径以实际安装为准：
# 先用这条命令查一下路径再 cp
pacman -Ql v2ray-rules-dat | grep geosite
sudo cp /usr/share/v2ray/geosite.dat ~/.config/mihomo/geosite.dat 

# 归属自己，方便以后瞎改
sudo chown $USER:$USER ~/.config/mihomo/geoip.metadb ~/.config/mihomo/geosite.dat 
```

> `Country.mmdb` 是 MaxMind 格式, Clash 原版就是用它, `GEOIP,CN` 规则完全够用
> 装完后 mihomo 不会再触发联网下载

---

## 0x05 systemd 用户服务

`mihomo` 不需要 root, 所以可以配置为用户服务

创建文件 `~/.config/systemd/user/mihomo.service`:

```ini
[Unit]
Description=mihomo proxy core
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/bin/mihomo -d /home/你的用户名/.config/mihomo
Restart=on-failure
RestartSec=3

[Install]
WantedBy=default.target
```

> systemd 不展开 `~`, 所以 ExecStart 的 `-d` 后面必须是绝对路径

确保你的yaml配置文件, geo数据文件全部无误后, 启动服务, 并且看日志确认没报错

```bash
systemctl --user daemon-reload
systemctl --user enable --now mihomo 

journalctl --user -u mihomo -n 20 --no-pager
```

如果你不会看日志, 那就用一条简单的命令测试喽~

```bash
curl -x http://127.0.0.1:7890 -s -o /dev/null -w '%{http_code} %{time_total}s\n' https://www.google.com
```

只要 http_code 是 200, 那你就成功啦~

---

## 0x06 TUN 模式与权限

TUN 模式在系统层建一张虚拟网卡, 所有流量自动进代理, 可以做到应用全覆盖

在 `config.yaml` 里面添加配置:

```yaml
tun:
  enable: true
  stack: mixed
  auto-route: true
  auto-redirect: false
  auto-detect-interface: true
  dns-hijack:
    - any:53
  device: Mihomo 
```

以及 `dns`:

```yaml
dns:
  enable: true
  ipv6: false
  enhanced-mode: fake-ip
  fake-ip-range: 198.18.0.1/16
  nameserver:
    - 223.5.5.5
    - 119.29.29.29
```

然后给 `mihomo` 文件能力授权:

```bash
sudo setcap cap_net_admin,cap_net_raw=ep /usr/bin/mihomo
getcap /usr/bin/mihomo    # 确认显示 cap_net_admin,cap_net_raw=ep
```

`pacman` 的升级可能会冲掉 `setcap`, 我们添加一个 hook, 自动回补

创建文件 `/etc/pacman.d/hooks/mihomo-cap.hook`:

```ini
[Trigger]
Operation = Upgrade
Operation = Install
Type = Path
Target = usr/bin/mihomo

[Action]
Description = Restore CAP_NET_ADMIN on mihomo
When = PostTransaction
Exec = /usr/bin/setcap cap_net_admin,cap_net_raw=ep /usr/bin/mihomo 
```

重启核心, 看虚拟网卡:

```bash
systemctl --user restart mihomo
ip link show Mihomo 
```

---

## 0x07 WebUI 配置

> 你可以直接使用在线版面板: [metacubexd](https://metacubex.github.io/metacubexd/), 这样就不需要 `metacubexd-bin` 那个本地包了

`metacubexd-bin` 其实就是提供了一个轻量的 HTTP 应用, 所以你可以用任何方式开启一个 HTTP SERVER 并且访问, 例如这样:

```bash
python3 -m http.server 8090 --bind 127.0.0.1 --directory /usr/share/metacubexd 
```

然后用浏览器打开 `http://127.0.0.1:8090`, 后端地址填 `http://127.0.0.1:9090`, 就能看到这个 WebUI

不过我的做法稍微有点不同, 我选择在本地配置一个反向代理, 这样可以通过域名直接访问

首先在 `/etc/hosts` 中添加一行本地 DNS:

```ini
127.0.0.1 proxy.example.com 
```

> 这个域名随便编一个你最喜欢的就行

然后安装反向代理使用的软件包, 我这里演示 `caddy`:

```bash
sudo pacman -S caddy 
```

编辑配置文件 `/etc/caddy/Caddyfile`

```bash
http://proxy.example.com {
    handle /api/* {
        uri strip_prefix /api
        reverse_proxy 127.0.0.1:9090
    }
    root * /usr/share/metacubexd
    file_server
}
```

然后启动 `caddy` 服务

```bash
sudo systemctl enable --now caddy 
```

这样打开 `http://proxy.example.com` 就可以直接访问面板了

但是现在后端地址每次都要手动填, 我们可以配置默认后端地址

编辑文件 `/usr/share/metacubexd/config.js`

```js
window.__METACUBEXD_CONFIG__ = {
  defaultBackendURL: 'http://proxy.example.com/api',
  githubToken: '',
}
```

> 这里要填 `/api` 的原因是: `Caddyfile` 里的 `handle /api/*` 会把前缀剥掉后反代到 9090
> 所以 `http://proxy.example.com/api` 等价于 `http://127.0.0.1:9090`
> 这样请求是同源的, 浏览器不拦截

如果你走 python http.server 面板, 或者 metacubex.github.io 遇到了跨源问题

那是因为 origin 是 `127.0.0.1:8090 / metacubex.github.io`, 但 API 是 `127.0.0.1:9090`

需要编辑 mihomo 的 `config.yaml` 放行:

```yaml
external-controller-cors:
  allow-origins:
    - http://127.0.0.1:8090
    - https://metacubex.github.io
  allow-private-network: true
```

改完重启: `systemctl --user restart mihomo`

---

## 0x08 从 mihomo-party 迁移过来

如果你和我一样再用 `mihomo-party`, 现在需要迁移, 那么大概分这四步:

#### 1) 拿配置

```bash
# party 的订阅文件位置（config.yaml 是它的主配置，订阅在 profiles/ 下）
cp ~/.config/mihomo-party/profiles/<订阅ID>.yaml ~/.config/mihomo/config.yaml
```

这是将静态副本拷贝出来, 之后订阅更新要重新弄, 或者你可以自己写一个自动更新脚本

#### 2) 核心切换

```bash
pkill -x mihomo-party
```

先杀掉进程(因为会抢端口), 然后按照前面的步骤把裸核跑起来

#### 3) 清理残留

```bash
sudo pacman -Rns clash-party-bin
# 用户配置残留（Electron 缓存 + 旧配置，几百 MB）
rm -rf ~/.config/mihomo-party
```

> 这一步想做就做吧) 我因为洁癖很严重, 所以家目录和系统包必须保持干净
> 配置留着也可以, 以后包装回来就能用

---

## 0x09 巧思

> 其实是我爱瞎折腾的老毛病犯了

#### 输入一条命令启动 WebUI

可以创建一个 `~/.local/bin/lxy` 文件, 里面写上:

```bash
#!/usr/bin/env bash
xdg-open http://proxy.example.com
```

这样直接在命令行输入 `lxy` 命令就能打开WebUI (嘻嘻)

#### 自己修改 WebUI 逻辑

你完全可以把 `/usr/share/metacubexd` 下面那套配置复制到自己的家目录下面, 然后随便修改逻辑, 添加功能

然后只需要在 `Caddyfile` 里面改一下 root 根路径就行了, 很好玩的

#### fallback 自动选择组

`select` 组需要手动选择节点, `fallback` 组可以自动测活, 一个节点挂了就换另一个

直接在 `config.yaml` 里面配置组就行:

```yaml
proxy-groups:
  - name: 自动选择喵
    type: fallback
    proxies: [节点1, 节点2, ...]  # 放你常用的几个
    url: http://www.gstatic.com/generate_204
    interval: 300
    tolerance: 50
```

然后你就可以在 WebUI 切换到这个代理组啦

---

## 0x10 结语

这套方案至少能省下来 800MB 内存

不过最大的收益还是 **内核, 配置, 界面** 完全解耦了, 自己以后想怎么改就怎么改

如果我喜欢的话, 也可以自己写一个 GUI / TUI 来用, 甚至整合到 `quickshell` 里面

Arch 用户的快乐, 大概就是这样
