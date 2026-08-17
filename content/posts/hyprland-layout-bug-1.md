---
id: 21
title: "Hyprland 切换布局时闪退问题修复"
slug: "hyprland-layout-bug-1"
summary: "在 hyprland-git 0.54.0 中，调用 hyprctl getoption general:layout 会触发 IPC 路径中的 std::format/JSON 处理 bug，导致 Hyprland 出现 SIGABRT 崩溃。该问题属于上游缺陷。临时通过状态文件替代查询逻辑，成功规避崩溃并实现布局切换"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/hyprland-layout-bug%231.jpg"
status: "published"
is_featured: true
theme: "Linux系统 (linux)"
tags: ["scrolling", "dwindle", "hyprland", "archlinux", "linux"]
view_count: 465
created_at: "2026-03-21 11:31:50"
updated_at: "2026-08-16 16:32:12"
source: "https://blog.yeastar.xin/posts/hyprland-layout-bug-1"
---

# Hyprland 问题记录

---

## 2026-03-21 | hyprctl getoption 触发 SIGABRT 崩溃

### 现象

更新 `hyprland-git` 到最新版本(`hyprland-git 0.54.0.r108.8726a7363-1`)后，按 `SUPER+S` 切换布局(dwindle -> scrolling)时 Hyprland 稳定崩溃，随后 watchdog 触发安全模式启动，再次崩溃。

---

### 如何发现问题

**第一步：确认崩溃记录**

```bash
coredumpctl list
```

发现今天（2026-03-21）有多条 Hyprland SIGABRT 记录：

```
Sat 2026-03-21 10:31:49 CST  1058  SIGABRT  /usr/bin/Hyprland  24.8M
Sat 2026-03-21 10:31:50 CST  1499  SIGSEGV  /usr/lib/xdg-desktop-portal-hyprland
Sat 2026-03-21 10:31:50 CST 15459  SIGABRT  /usr/bin/Hyprland   1.0M
Sat 2026-03-21 10:32:32 CST 15696  SIGABRT  /usr/bin/Hyprland  22.0M
...
```

体积小（1M）的是安全模式崩溃，体积大（24M）的是主进程崩溃，从最大的 PID 开始分析。

**第二步：查看崩溃堆栈**

```bash
coredumpctl info 1058
```

关键堆栈（第一次崩溃）：

```
#5  _Z17escapeJSONStringsRK...       ← 崩溃点
#6  n/a (/usr/bin/Hyprland)
#7  CHyprCtl::getReply               ← IPC 处理
#10 wl_event_loop_dispatch
#12 main
```

初步判断：`hyprctl getoption` 的 JSON 输出路径有 bug。

**第三步：修复后仍然崩溃，再次查堆栈**

去掉 `-j` 改为纯文本解析后，SUPER+S 仍然崩溃。查新的崩溃（PID 18433）：

```bash
coredumpctl info 18433
```

关键堆栈：

```
#5  n/a (libc.so.6)                  ← __chk_fail / format 异常
#6  std::__format::_Sink::_M_write   ← std::format 内部
#10 std::vformat                     ← 格式化崩溃
#11 n/a (/usr/bin/Hyprland)
#12 CHyprCtl::getReply               ← 同一路径
```

确认：不是 `-j` 参数的问题，而是 `hyprctl getoption` 本身触发了 `CHyprCtl::getReply` 中的 `std::format` bug。

---

### 问题分析

`SUPER+S` 的切换脚本使用 `hyprctl getoption general:layout` 查询当前布局，这个 IPC 请求在新版 `hyprland-git` 的 `CHyprCtl::getReply` 中触发了 `std::format` / `escapeJSONStrings` 的 bug，导致 SIGABRT。

崩溃链：

1. `SUPER+S` 触发 shell 脚本
2. 脚本调用 `hyprctl getoption general:layout`
3. Hyprland IPC → `CHyprCtl::getReply`
4. 内部 `std::format` 格式化响应时越界/断言失败
5. `abort()` → SIGABRT
6. Watchdog 检测到崩溃，以安全模式重启
7. 安全模式下配置解析同样崩溃（config 路径问题）

这是 `hyprland-git` 上游 bug，与本地配置无关。

---

### 解决方案

绕开 `hyprctl getoption`，改用状态文件记录当前布局：

**修复前（hyprland.conf）：**

```bash
bind = $mainMod, S, exec, \
  current=$(hyprctl getoption general:layout -j | jq -r '.str'); \
  if [ "$current" = "dwindle" ]; then \
    hyprctl keyword general:layout scrolling && notify-send "切换到 scrolling 布局"; \
  else \
    hyprctl keyword general:layout dwindle && notify-send "切换到 dwindle 布局"; \
  fi
```

**修复后（hyprland.conf）：**

```bash
bind = $mainMod, S, exec, \
  state_file=/tmp/hypr_layout_state; \
  current=$(cat "$state_file" 2>/dev/null || echo "dwindle"); \
  if [ "$current" = "dwindle" ]; then \
    hyprctl keyword general:layout scrolling && echo scrolling > "$state_file" && notify-send "切换到 scrolling 布局"; \
  else \
    hyprctl keyword general:layout dwindle && echo dwindle > "$state_file" && notify-send "切换到 dwindle 布局"; \
  fi
```

---

### 注意事项

- 状态文件 `/tmp/hypr_layout_state` 在重启后丢失，默认回退到 `dwindle`，与 `hyprland.conf` 中 `general:layout` 的默认值保持一致即可。
- 这应该是 `hyprland-git` 上游 bug，等待上游修复后可恢复使用 `hyprctl getoption`。
- 排查命令速查：
  ```bash
  coredumpctl list                  # 查看崩溃历史
  coredumpctl info             # 查看指定崩溃的详细堆栈
  journalctl -b 0 | grep -i hypr   # 查看本次启动的 hyprland 日志
  ls $XDG_RUNTIME_DIR/hypr/        # 查看 hyprland 运行时日志目录
  ```
