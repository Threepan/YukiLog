// ================================
// 兼容配置导出层
// ================================
// 说明：
// - 真实配置源位于项目根目录 `yukilog.config.ts`
// - 本文件仅负责向现有代码提供稳定导出，避免大量改动引用路径

export {
  siteConfig,
  navItems,
  designTokens,
  contentConfig,
  yukilogConfig,
} from "../../yukilog.config";

// API 基础 URL（运行时环境变量）
export const API_BASE_URL =
  import.meta.env.PUBLIC_API_URL || "http://localhost:3000";
