---
id: 16
title: "Hyprland 下 LinuxQQ 剪贴板、fcitx 输入法与窗口透明三连问题排查"
slug: "hyprland-linuxqq-fcitx"
summary: "记录在 Hyprland (Wayland) 环境下使用 LinuxQQ 时遇到的三个连锁问题：剪贴板无法复制、fcitx 无法输入中文、fcitx 候选窗口半透明，以及对应的解决方案"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/hyprland-linuxqq-fcitx.jpg"
status: "published"
is_featured: true
theme: "Linux系统 (linux)"
tags: ["linuxqq", "archlinux", "linux"]
view_count: 868
created_at: "2026-02-24 01:21:10"
updated_at: "2026-08-17 15:51:21"
source: "https://blog.yeastar.xin/posts/hyprland-linuxqq-fcitx"
---

## 问题

我在使用 `linuxqq` 的时候一直都会遇到一个很恶劣的问题:

不管是 **右键复制** 还是 **Ctrl + c** 都没办法复制内容

我的剪贴板是 **cliphist** + **wl-copy**, 桌面环境为 `hyprland (wayland)`

---

## 解决方案

后来我发现给 `linuxqq` 添加 `-ozone-platform=x11` 可以让剪贴板恢复正常

这个启动参数的作用是让 `linuxqq` 恢复到 `x11` 模式运行

---

## 原理

`linuxqq` 是基于 **Electron** 的应用。Electron 在 Wayland 下默认会尝试使用 Wayland 原生协议（通过 `ozone` 平台层），但 Wayland 的剪贴板机制与 X11 有本质区别：

- **X11** 下剪贴板是全局共享的，任何应用都可以直接读写
- **Wayland** 下剪贴板访问受到协议限制，需要通过 `wl-clipboard` 等工具，且只有持有焦点的窗口才能访问

当 Electron 的 Wayland 后端与 `cliphist` / `wl-copy` 的集成存在兼容性问题时，就会出现复制失效的情况。通过 `-ozone-platform=x11` 强制让 linuxqq 走 **XWayland**（X11 兼容层），绕过 Wayland 剪贴板协议，问题得以解决。

> 如果你不确定自己当前跑的是 Wayland 还是 X11，可以用以下命令确认：
>
> ```bash
> echo $XDG_SESSION_TYPE
> # 输出 wayland 或 x11
>
> # 也可以检查 WAYLAND_DISPLAY 变量是否有值
> echo $WAYLAND_DISPLAY
> # 有输出（如 wayland-0）说明是 Wayland，为空则是 X11
> ```
>
> 查看当前剪贴板内容：
>
> ```bash
> # Wayland 下
> wl-paste
>
> # X11 / XWayland 下
> xclip -o
> ```

---

## 新的问题

解决了剪贴板后, 我发现我的输入法无法工作(无法输入中文)

调查过后是缺少了环境变量。为什么切换到 X11 模式后输入法就失效了？

在 Wayland 模式下，fcitx5 通过 Wayland 输入法协议（`text-input-v3`）与应用通信，不依赖这些环境变量。但切换到 XWayland / X11 模式后，应用改为通过 **XIM / GTK IM Module** 等传统机制与输入法通信，这些机制需要靠环境变量来告诉应用"去找 fcitx"，缺少这些变量应用就不知道该用哪个输入法。

我的解决办法是直接编辑 `desktop` 文件。由于系统级的 desktop 文件不建议直接修改（包管理器更新时会被覆盖），先将其复制到用户目录：

```bash
cp /usr/share/applications/qq.desktop ~/.local/share/applications/qq.desktop
```

用户目录下的 desktop 文件优先级更高，且不会被包管理器覆盖。然后编辑这个副本：

```ini
# ~/.local/share/applications/qq.desktop
...
Exec = env GTK_IM_MODULE=fcitx linuxqq -ozone-platform=x11 %U
...
```

AI 给我的完整环境变量是 `GTK_IM_MODULE=fcitx QT_IM_MODULE=fcitx XMODIFIERS=@im=fcitx SDL_IM_MODULE=fcitx`

不过在我的电脑上只需要配置 `GTK_IM_MODULE` 就可以了，因为 linuxqq 是基于 GTK 的 Electron 应用，只走 GTK 的输入法通道

---

## 新的新的问题

接下来我发现 `fcitx` 的前端窗口变成半透明样式了, 原因是我在 hyprland.conf 中做了以下配置:

```ini
decoration {
    inactive_opacity = 0.6
}
```

`inactive_opacity` 会对所有非焦点窗口生效。之前在 `wayland` 模式时没什么问题，是因为 fcitx5 的候选窗口走的是 Wayland 原生协议，Hyprland 对它的处理方式不同。但切换到 X11 模式后，fcitx 的候选窗口变成了一个普通的 XWayland 浮动窗口，`inactive_opacity` 就命中了它。

接着我打算为 fcitx 单独书写透明规则，首先使用 `hyprctl` 获取 fcitx 的前端窗口信息：

```bash
sleep 3 && hyprctl clients
```

> 这边要暂停三秒是因为留出时间来打字触发窗口，fcitx 的候选窗口只有在输入时才会出现

然后我获得了以下信息:

```bash
Window 56483ec59fa0 -> Fcitx5 Input Window:
    ...
    class: fcitxfcit
    title: Fcitx5 Input Window
    initialClass: fcitxfcit
    initialTitle: Fcitx5 Input Window
    xwayland: 1
    ...
```

接着我使用了很多种匹配规则都无法命中 fcitx：

```ini
# 使用 class
windowrule = opacity 1.0 override, match:class ^(fcitxfcit)$
# 使用 initialClass
windowrule = opacity 1.0 override, match:initial_class ^(fcitxfcit)$
```

这两条规则失效的原因可能是 fcitx 候选窗口的 class 在某些版本或时机下并不稳定，而 `initial_title` 是窗口创建时就固定下来的，不会随状态变化，所以更可靠。最后是使用 `initial_title` 匹配到的：

```ini
windowrule = opacity 1.0 override, match:initial_title ^(Fcitx5 Input Window)$
```

---

## 最后

至此所有问题都解决了, 捣鼓 arch 大概就是这样的, 一个问题能衍生出好多问题来
