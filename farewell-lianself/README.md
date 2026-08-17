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

## 外部依赖

克隆保留了源站原有的两处远程引用（与线上行为一致，未做本地化改写）：

| 位置 | 地址 | 作用 | 离线时的表现 |
| --- | --- | --- | --- |
| `index.html:6` | `https://list.yeastar.xin/d/%E6%81%8B/avatar-modified.png` | favicon | 标签页图标显示为浏览器默认图标 |
| `css/common.css:1` | `https://fonts.googleapis.com/css2?family=Fira+Code...` | `@import` 引入 Fira Code 字体 | 回退到 `monospace`，布局与交互不受影响 |

除这两项外，页面的结构、样式、脚本与全部数据均已本地化，断网也能打开并正常切换四种状态。

## 校验记录

- 9 个文件的字节数均与源站 `Content-Length` 一致（见上表）。
- 三个 JS 文件通过 `node --check`；五个 CSS 文件花括号配对平衡。
- 用 jsdom 实际加载 `index.html` 验证运行时行为：
  - 初始渲染 DEFAULT 状态 11 行，标题栏显示 `lian.inc`；
  - 点击「最小化」后 `body` class 变为 `state-vitrimura`，标题栏变为 `State_Vitrimura`；
  - 窗口 `blur` 时 `document.title` 变为 `not for your eyes ...`，`focus` 时恢复 `Lianself`。
