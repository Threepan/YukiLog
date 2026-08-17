# Lianself — farewell.yeastar.xin 离线克隆

<https://farewell.yeastar.xin/> 的完整离线副本，抓取时间 **2026-08-17 (CST)**。
页面标题为 `Lianself`，窗口失焦时变为 `not for your eyes ...`。

## 文件清单

所有文件均已与源站 `Content-Length` 逐字节校验一致。

| 文件 | 字节 | 说明 |
| --- | --- | --- |
| `index.html` | 1410 | 页面骨架 |
| `css/common.css` | 2795 | 公共样式与状态切换基础 |
| `css/default/style.css` | 881 | DEFAULT 状态样式 |
| `css/aurora/style.css` | 2473 | AURORA 状态样式 |
| `css/vitrimura/style.css` | 9748 | VITRIMURA 状态样式 |
| `css/nostofobia/style.css` | 2617 | NOSTOFOBIA 状态样式 |
| `js/data.js` | 6358 | 各状态的代码行数据 `CODE_DATA` |
| `js/main.js` | 6543 | 状态机、渲染、事件绑定 |
| `js/vitrimura.js` | 5618 | VITRIMURA 状态的碎片（shard）特效 |

## 本地预览

```bash
python3 -m http.server 8080 --directory .
# 然后打开 http://127.0.0.1:8080
```

## 行为说明

站点是一个四状态的交互装置：

| 状态 | 标题 | 触发方式 |
| --- | --- | --- |
| `DEFAULT` | `lian.inc` | 点击 `.window` / `.shard` 之外的区域 |
| `AURORA` | `State_Aurora` | 窗口「关闭」按钮 |
| `VITRIMURA` | `State_Vitrimura` | 窗口「最小化」按钮 |
| `NOSTOFOBIA` | `State_Nostofobia` | 窗口「最大化」按钮 |

- 再次点击当前状态对应的按钮会切回 `DEFAULT`。
- `body` 的 class 为 `state-<小写状态名>`，各状态的样式表据此生效。
- `CODE_DATA` 中每个状态各有一组 `{line, html, source}` 记录：DEFAULT 11 行、AURORA 5 行、VITRIMURA 7 行、NOSTOFOBIA 8 行。

## 唯一的外部依赖

favicon 仍指向远程地址 `https://list.yeastar.xin/d/%E6%81%8B/avatar-modified.png`；
除此之外该克隆完全自包含，可离线打开。
