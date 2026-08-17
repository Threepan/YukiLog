---
id: 1
title: "archlinux 最小化安装指南"
slug: "archlinux-guide"
summary: "一份面向新手的 Arch Linux 完整安装教程，从 UEFI 引导到 Btrfs 分区，从系统配置到哲学思考。不仅教你如何安装，更让你理解\"为什么这样做\"，体验亲手赋予操作系统生命的过程。"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/archlinux-guide.jpg"
status: "published"
is_featured: true
theme: "Linux系统 (linux)"
tags: ["系统安装", "教程", "archlinux", "linux"]
view_count: 1096
created_at: "2026-02-08 21:39:28"
updated_at: "2026-08-17 15:50:16"
source: "https://blog.yeastar.xin/posts/archlinux-guide"
---

> 如果本教程与 wiki 有出入, 请永远以 wiki 为准

[Arch Linux - ArchWiki](https://wiki.archlinux.org/title/Arch_Linux)

## 🧭 我所理解的 Arch 哲学

在正式开始安装之前, 我希望向你介绍 **我认为的 Arch** 是一个怎样的系统

Arch 不「贴心」

Arch 不「安全」

Arch 甚至不「友好」

#### 它要求的只有三件事:

阅读, 思考, 为自己的系统负责

具体来说就是:

查 wiki, 看文档, 在系统坏掉时不怪系统

#### 我为什么要教你安装 Arch

不是为了 "让你觉得自己很酷" "让你加入某个圈子"

而是为了让你体验一次

> **从零开始，亲手把一个操作系统「活过来」**

哪怕你最后不用 Arch, 这段经历也会改变你看待 linux, 看待软件, 甚至看待 "工具与人" 的方式

#### 🧩 结语

Arch 不适合所有人

但如果你:

* 讨厌 "系统替你做决定"
* 想真正理解你每天使用的系统
* 愿意为自由付出时间

那么 Arch 不会背叛你, 他只会如实的回应你的每一个选择

当你看到 Login 提示符时, 那就是你思考的结果

---

## 🧰 安装教程

如果你准备好了, 接下来我会手把手带你看到那个 `Login:`

#### 🧩 安装 - 准备工作

##### (1) **物理机, 虚拟机 与 WSL:**

虽然本教程侧重于**物理机安装**, 但你可能在不同的环境下尝试:

* **物理机安装:** 最真实的体验, 性能最强, 但风险最高. 你需要准备一个 **容量 > 2GB** 的U盘 (注意: U盘会被格式化, 数据请提前备份)
* **虚拟机 (VMWare/VirtualBox):** 最安全, 你不需要U盘, 直接挂载 `.iso` 文件即可, 通常也不需要调试网络 (因为有虚拟网卡). 但是在创建虚拟机时请开启 **EFI引导模式**, 否则与本教程不符
* **WSL (Windows Subsystem for Linux):** 这是一个在 Windows 里跑的 **子系统**, 如果你想学习 Linux 架构, WSL 会屏蔽掉底层硬件细节, 但是他非常适合临时体验 Linux

##### (2) **电脑是如何醒来的**

理解开机过程, 能够帮你搞清楚 为什么我们要折腾 BIOS:

1. **通电 (Power On):** 硬件开始供电
2. **固件初始化 (UEFI/BIOS):** 这是主板自带的一段小软件, 负责 **硬件上电自检, 初始化CPU, 创建最小运行环境, 找到并启动引导程序 (bootloader)**
3. **启动内核:** 将 Linux 内核加载进内存
4. **进入系统**

##### (3) **你是 UEFI 还是 BIOS ?**

现代电脑几乎都使用 **UEFI**, 他是传统 BIOS 的升级版. **Arch Linux 的安装方式在 UEFI 和 BIOS 下不同**, 本教程基于主流的 **UEFI + GPT**

**如何查看当前系统的模式?**

* **Windows 用户:** 按下 `Win + R`, 输入 `msinfo32`, 在 "系统摘要" 里找 "BIOS模式". 如果是"URFI"那就对了, 如果是 "Legacy", 说明你的电脑很老
* **Linux 用户:** 查看路径 `/sys/firmware/efi` 是否存在 (在终端输入 `ls /sys/firmware/efi`), 如果存在, 就是 URFI

##### (4) **获取 Arch ISO 与 磁链**

前往 [Arch Linux 下载页面](https://archlinux.org/download/). 你会看到很多国家镜像, 但最醒目的是 **Magnet (磁链)**

> **科普：磁链是什么**
>
> 磁链 (Magnet Link) 不依赖某个固定的服务器, 而是通过 P2P 协议从全球其他正在下载的人那里获取数据
>
> * **工具:** 你可以使用 BitComet, qBittorrent, 甚至迅雷, 百度网盘
> * **优点:** 不限速, 下载人数越多越快

##### (5) **烧录启动盘**

下载好 `.iso` 后, 我们需要把他 "烧" 进 U盘

* **Windows 推荐工具: Rufus**
  * **分区类型:** 选 **GPT**
  * **目标系统类型:** 选 **UEFI (非 CSM)**
  * **重要提示:** 点击开始后, 如果弹出提示, 请务必选择 **DD模式** 写入, 因为 Arch 的镜像本身就是很完整的
* **Linux 推荐工具: Ventoy**
  * 你只需要把 U盘 格式化为 Ventoy, 然后把 `.iso` 文件拖进去即可

##### (6) **BIOS 避坑指南**

插入 U盘, 重启电脑后, 在 Logo 出现时狂按 `F2` (不同电脑的按键可能不同) 进入 BIOS 设置, 请务必确认一下几点:

1. **关闭安全启动 (Secure Boot):** 这是微软的一种保护机制, 但他会阻止未授权的 Linux 引导, 必须设置为 `Disabled`
2. **SATA 模式:** 如果你有多块硬盘, 确保 SATA 模式不是 `RAID` 而是 `AHCI` (现代 NVMe 固态通常默认即可)
3. **调整启动顺序**: 将你的 U盘 (通常带有 `UEFI: ` 前缀) 设为第一启动项
4. **保存并重启**

---

#### 🧩 安装 - 环境配置

当你重启电脑并从 U盘 启动后, 你会看到一个菜单, 选择第一项 `Arch Linux install medium (x86_64, UEFI)`

当屏幕停止滚动, 出现 `root@archiso ~ #` 时, 你已经成功进入了 Arch 的临时内存系统

##### (0) 【可选】**确认引导模式**

在终端输入 `ls /sys/firmware/efi/efivars`, 如果你看到了一堆文件, 说明你确实是以 **UEFI** 模式启动的

##### (1) 连接网络

Arch 本身不包含所有软件包, 他需要通过网络实时拉去最新的软件

- **如果你插了网线:** 通常系统会通过 DHCP 自动获取 IP, 输入 `ping -c 4 google.com` 或 `ping -c 4 baidu.com` (如果你是国内用户, ping 不到 google 是完全正常的, ping 到 百度 就行)
- **如果你使用 Wi-Fi:** 我们需要使用 `iwctl` 这个交互式工具

Wi-Fi 连接步骤

1. 输入 `iwctl` 进入交互页面
2. `device list`: 查看网卡名称 (通常是 `wlan0`)
3. `station wlan0 scan`: 让网卡扫描附近的信号
4. `station wlan0 get-networks`: 列出搜到的 Wi-Fi 名字
5. `station wlan0 connect <你的WiFi名称>`: 输入密码连接
6. `quit`: 连接成功后退出工具

**避坑指南:** Arch 默认不支持中文显示, 如果你的 WiFi 名称有中文, 这里会显示成乱码或链接失败, 建议把路由器名称改回英文

##### (2) 更新系统时间

如果你的系统时间与真实时间相差太大, 会导致下载软件包时由于 "证书过期" 而报错

1. 开启网络时间同步

```bash
timedatectl set-ntp true
```

2. 设置时区 (亚洲/上海)

```bash
timedatectl set-timezone Asia/Shanghai
```

##### (3) 刷新镜像源 (mirrorlist)

这是最影响下载速度的一步, 默认的镜像源可能在国外, 速度极慢. 我们使用 `reflector` 自动挑选最快的国内镜像:

1. 生成镜像源列表

```bash
reflector --country China --protocol https --latest 10 --sort rate --save /etc/pacman.d/mirrorlist
```

这条命令的意思是: 在中国镜像站中, 寻找 10 个最近同步过的 HTTPS 服务器, 按速度排序, 并把结果存入配置文件

2. 查看生成结果

```bash
cat /etc/pacman.d/mirrorlist
```

---

#### 🧩 安装 - 磁盘分区

这是整个安装过程中最硬核的部分, 给我们的硬盘分个区, 规定不同的文件住在不同的分区里

> **科普：Btrfs 是什么**
>
> **Btrfs** 是一种先进的文件系统, 传统文件系统 (如 Ext4) 像一张纸, 改错了只能擦掉重写
>
> **Btrfs** 像一本书, 他的特性是:
>
> * **子卷 (Subvolumes):** 你可以把根目录 `/` 和用户目录 `/home` 分为不同的子卷
> * **快照 (Snapshots):** 如果你玩坏了系统, 可以通过快照恢复到之前还没坏的状态

> **科普：EFI 是什么**
>
> **EFI 分区** 是主板固件 (UEFI) 唯一能读懂的分区, 你可以理解为这是开机的第一站
>
> 他里面存放着 **引导程序 (Bootloader, 如 GRUB 或 systemd-boot)**
>
> UEFI 非常简单, 他只认识最原始的 FAT32 格式, 如果没有 EFI 分区, 你的电脑连操作系统都找不到

##### (1) 分区规划

我们将采用 **GPT** 分区表, 这是现代电脑的标准


| 分区名        | 挂载点  | 建议大小   | 文件系统类型 | 说明                               |
| ------------- | ------- | ---------- | ------------ | ---------------------------------- |
| **EFI 分区**  | `/boot` | 512MB      | FAT32        | 存放引导程序的"小隔间"             |
| **Swap 分区** | `无`    | 8GB - 16GB | Linux swap   | 虚拟内存, 内存不够时会使用这一部分 |
| **根分区**    | `/`     | 剩余所有   | Btrfs        | 存放你的整个系统和所有文件         |

##### (2) 配置分区

分区工具有很多种, 你可以选择命令行交互页面 `fdisk`, 也可以选择图形化页面的 `cfdisk`

输入 `lsblk` 找到你的硬盘名称 (例如 `/dev/nvme0n1` 或 `/dev/sda`), 然后:

```bash
fdisk /dev/nvme0n1  # 使用 fdisk
cfdisk /dev/nvme0n1 # 使用 cfdisk
```

**使用 cfdisk**

1. **Select Label Type:** 选择 `gpt`
2. **新建 EFI:** `New` -> `512M` -> `Type` 选 `EFI System`
3. **新建 Swap:** `New` -> `16G` -> `Type` 选 `Linux swap`
4. **新建根分区:** `New` -> `剩余空间` -> `Type` 默认 `Linux root (x86-64)`
5. **写入:** 写入 `Write` 并输入 `yes`, 最后 `Quit` 退出

**使用 fdisk**

在 **fdisk** 的提示符下

1. **创建 GPT 分区表:** `g` (设置分区类型)
2. **新建 EFI:** `n` (新建分区) -> `1` (设为一号) -> `` (回车键) -> `+512M` (增加 512 M), **设置分区类型:** `t` (设置类型) -> `1` (一号分区) -> `1` (EFI System)
3. **新建 Swap:** `n` -> `1` -> `` -> `+16G`, **设置分区类型:** `t` -> `2` -> `19` (Linux swap)
4. **新建根分区:** `n` -> `3` -> `` -> ``
5. **查看分区无误:** `p`
6. **写入分区表并退出:** `w`

如果磁盘之前有数据, `fdisk` 会询问是否删除旧签名, 这里一定要选 `Y`

##### (3) 格式化

这一步才是将分区数据实际写入磁盘, 所以如果上一步你做错了也不要紧, **不写入/清空重分** 就好了

1. 格式化 EFI 分区 (一号分区)

```bash
mkfs.vfat -F 32 /dev/nvme0n1p1
```

2. 格式化并启用 Swap 分区

```bash
mkswap /dev/nvme0n1p2
swapon /dev/nvme0n1p2
```

3. 格式化根分区为 Btrfs (myarch 你可以随便取名)

```bash
mkfs.btrfs -L myarch /dev/nvme0n1p3
```

##### 【可选】Btrfs 子卷挂载

为了实现 系统 和 数据 的分离, 我们不在 Btrfs 的物理根上直接安装, 而是建立子卷

当你给系统做备份的时候, 你可以只备份 系统核心卷, 不必备份 用户卷 和 日志卷, 节省空间且逻辑清晰

**挂载物理分区并创建子卷**

```bash
mount /dev/nvme0n1p3 /mnt
btrfs subvolume create /mnt/@
btrfs subvolume create /mnt/@home
btrfs subvolume create /mnt/@var
umount /mnt
```

**挂载子卷**

```bash
mount -o subvol=@ /dev/nvme0n1p3 /mnt
mkdir -p /mnt/{boot,home,var}
mount -o subvol=@home /dev/nvme0n1p3 /mnt/home
mount -o subvol=@var /dev/nvme0n1p3 /mnt/var
mount /dev/nvme0n1p1 /mnt/boot
```

##### 最后查看你的磁盘情况

```bash
lsblk -f
```

---

#### 🧩 安装 - 基础系统

现在的 `/mnt` 目录已经挂载好了我们的硬盘, 接下来我们要通过网络, 把最核心的系统文件下载并解压进去

##### (1) 安装基础软件包

我们使用 `pacstrap` 脚本来安装基础软件包

```bash
pacstrap -K /mnt base linux linux-firmware base-devel btrfs-progs networkmanager vim sudo
```

**软件包说明:**

* `base`: 最基础的系统环境
* `linux`: 操作系统内核
* `linux-firmware`: 各种硬件驱动的固件 (显卡, 声卡, 网卡等)
* `base-devel`: 使用源码编译软件的工具包
* `btrfs-progs`: Btrfs 文件系统管理硬盘的工具
* `networkmanager`: 网络管理器 (极其重要, 没有他就连不了网啦)
* `vim`: 文本编辑器, 你也可以选择 `nano` `neovim` 等喜欢的编辑器
* `sudo`: 权限管理工具

**让网络管理器开机自启**

```bash
systemctl enable NetworkManager
```

##### (2) 生成分区表

现在的磁盘挂载关系只存在于内存中, 如果你重启, 系统就会忘记他们, 我们需要把他们永久写入文件 `/etc/fstab` 中

1. 生成 fstab 文件 (使用 UUID 标识硬盘, 更稳定)

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

**检查生成结果**

```bash
cat /mnt/etc/fstab
```

##### 【可选】Btrfs 压缩优化

1. **手动修改 fstab:** 使用 vim 编辑器 `vim /mnt/etc/fstab` (关于 vim编辑器 的使用这里不做科普)
2. **添加参数:** 在 所有 `btrfs` 分区的参数项中 (通常是在 `relatime` 后面), 手动加上 `compress=zstd:3`

**就像这样:**

```shell
UUID=da9c58a4-d828-45b5-a42e-7396218da611	/         	btrfs     	rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvol=/@	0 0

UUID=da9c58a4-d828-45b5-a42e-7396218da611	/home     	btrfs     	rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvol=/@home	0 0

UUID=da9c58a4-d828-45b5-a42e-7396218da611	/var      	btrfs     	rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvol=/@var	0 0
```

---

#### 🧩 安装 - 配置系统

现在我们要进入系统进行配置

##### (1) 进入新系统 (Chroot)

```bash
arch-chroot /mnt
```

如果你看到提示符变了, 说明你已经从 "安装盘" 切换到了 "硬盘里的系统"

我们将要在这里进行基本的系统配置

##### (2) 时区 与 语言 配置

1. 设置时区

```bash
ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
```

2. 同步硬件时钟

```bash
hwclock --systohc
```

**本地化设置**

1. 编辑 `/etc/locale.gen`, 找到 `en_US.UTF-8 UTF-8` 和 `zh_CN.UTF-8 UTF-8`, 删掉前面的 `#` 注释符号
2. 执行 `locale-gen` 生成语言包
3. 设置系统语言 `echo "LANG=en_US.UTF-8" > /etc/locale.conf`

**注意:** 在进入图形化界面前, 建议保持系统语言为英文, 否则命令行下中文会显示为方块

##### (3) 网络 与 主机名

可以给你的电脑取个名字:

```bash
echo "你的主机名" > /etc/hostname
```

##### (4) 创建用户 配置权限

**设置 Root 密码:** 输入 `passwd`, 按提示输入两次密码 (屏幕不会显示字符, 正常输入即可)

**创建普通用户:** Arch 不建议日常使用 Root 账号, 所以我们要创建一个普通账户使用

```bash
useradd -m -G wheel 你的用户名
passwd 你的用户名
```

**配置 Sudo:** 执行 `visudo` (如果你装的是 vim, 记得先执行 `export EDITOR=vim`), 找到这一行 `# %wheel ALL=(ALL:ALL) ALL` 删除开头的 `#` 号, 这样你刚才创建的用户就能用 `sudo` 执行高级命令了

##### (5) 安装引导程序 (Bootloader)

我们选择 **GRUB** 作为安装引导程序, 他的作用是: 在电脑刚启动时扫描操作系统，并负责将系统内核加载进内存。

1. 安装相关工具

```bash
pacman -S grub efibootmgr os-prober
```

2. 将引导程序安装到 EFI 分区

```bash
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
```

3. 生成配置文件

```bash
grub-mkconfig -o /boot/grub/grub.cfg
```

> **小提示:** `os-prober` 是用来检测电脑里有没有其他操作系统的, 如果你是双系统, 记得编辑文件 `/etc/default/grub` 最后加上 `GRUB_DISABLE_OS_PROBER=false` 再执行生成配置的命令

---

#### 🧩 安装 - 结束

到这一步, 系统就已经完全装入硬盘了, 我们现在要做的就是关机, 拔掉 U盘, 进入系统

1. 退出 Chroot 环境

```bash
exit
```

2. 递归卸载所有分区, 确保缓存写入磁盘

```bash
umount -R /mnt
```

3. 重启电脑

```bash
reboot
```

在屏幕黑掉之后, **拔掉你的 U盘** (你也可以进 BIOS 将硬盘选择为第一启动项)

如果一切顺利, 你就会看到 GRUB 的引导菜单, 选择 Arch Linux 并回车, 最后屏幕会定格在

```bash
主机名 login:
```

输入你之前创建的 **用户名**, 然后输入 **密码**, 当你看到这个提示符: `[用户名@主机名 ~]$`

**恭喜你, 你已经亲手赋予了一个操作系统生命**

---

#### 🧩 在这一切之后

这里有一些之后的建议, 可以帮助你快速入手 Arch:

##### (1) 经常更新系统

```bash
sudo pacman -Syu    # -S 同步云端软件包 -y 刷新数据库 -u 升级
```

Arch 是一个滚动更新的系统, 这意味着他几乎 **三天一大更, 一天一小更**, 每天进入系统后, 都推荐你马上进行更新

##### (2) Arch Linux CN 与 AUR

这是国内 Arch 用户必做的一步。Arch Linux CN 是一个由国内社区维护的非官方仓库，里面包含了大量符合中国用户习惯的软件(如 QQ、微信、腾讯会议)

**添加源**

```bash
# 编辑配置文件 sudo vim /etc/pacman.conf
# 在文件末尾添加以下内容 (推荐使用阿里云)
[archlinuxcn]
Server = https://mirrors.aliyun.com/archlinuxcn/$arch
# 下面这个是清华大学镜像源
# Server = https://mirrors.tuna.tsinghua.edu.cn/archlinuxcn/$arch
```

**安装密钥与核心工具**

```bash
sudo pacman -Sy
sudo pacman -S archlinuxcn-keyring  # 导入社区密钥
```

**AUR (Arch User Repository)** 是社区维护的软件仓库, 这里提供大量额外的软件

推荐安装 `paru` 作为新的包管理器代替 `pacman` (不推荐 `yay` 是因为 `paru` 基于 `rust` 语言, 我就喜欢 `rust`)

```bash
sudo pacman -S paru
```

以后你的所有 `sudo pacman ...` 都可以换成 `paru ...` 了, 这里宣传一下我自己做的包管理器前端:

一行命令即可安装, 关于这个软件的详情请看这里: [Github-lian](https://github.com/Yueosa/lian)

```bash
paru -S lian-bin
# 运行
lian
```

这个 TUI前端 非常适合新手小白使用, 还支援了 **AI自动总结**, 让你了解在 安装/卸载/升级 时系统发生了什么

##### (3) 图形环境

现在你看到的 Arch 大概只是黑底白字的命令行, 如果要看到好看的页面, 你还需要安装 **桌面环境**

**安装显卡驱动**

* **NVIDIA:** `sudo pacman -S nvidia-open-dkms`, 最近 nvidia 驱动彻底拥抱了开源内核, 所以我们安装这一个包
* **intel/AMD:** 通常已经包含在内核中, 但也建议安装 `sudo pacman -S mesa`

Arch 的魅力在于你可以自由选择桌面, 这里做几种推荐:

* **KDE Plasma:** 最像 Windows, 自带大量软件包开箱即用 (但你都用 arch 了, 应该不会愿意刚装的系统就多了一堆软件包吧?)
* **GNOME:** 像 MacOS, 简单, 圆滑, 稳定的桌面
* **Hyprland / Sway / niri:** 我心目中最 `linux` 的平铺式窗口管理器, 支持全键盘操作和极高程度自定义

> **Hyprland** 这类桌面是 "完全由用户搭建的桌面环境", 他的缺点只有一个: 学习路线陡峭
>
> 在这里你需要配置所有东西, 因为进入桌面后你往往只会看到一张壁纸, 其余什么页面都没有
>
> 状态栏, 应用管理器, 锁屏, 登录页面, 应用通知 ... 几乎全都要你自己去配置
>
> 就连 快捷键, 截图, 剪贴板, 输入法 这样最基本的需求也要你去管理

桌面具体如何配置, 这篇博客不做说明, 之后会专门有博客分享我的桌面环境配置的

##### (4) 网络代理

如果没有网络代理, 你在 linux 的世界将寸步难行

简单来说网络代理就是让你能自由访问国际互联网, 浏览外网的技术社区, 软件商店 ...

这里推荐使用 `mihomo-party` 作为代理客户端

```bash
paru -S mihomo-party-bin
```

关于网络代理的详情(例如终端代理), 我会另开一篇博客说明

---

## 🌟 最后

> 安装 **Arch** 并不是终点，而是你作为 "系统主人" 的起点。

现在的你已经拥有了一个干净、纯粹、完全由你控制的 Linux 环境。

虽然未来你可能会遇到 "系统滚挂" "驱动冲突" 等各种挑战，但请记住：

**阅读, 思考, 为自己的系统负责**

这就是 Arch 赋予你的自由

##### 作者的话

本篇博客非常啰唆, 如果影响到你 "以最快速度安装好", 我表示非常抱歉

但因为他面向小白撰写, 所以我想让你明白 **自己做的每一步**

***祝你在 Arch 的世界玩的开心!***
