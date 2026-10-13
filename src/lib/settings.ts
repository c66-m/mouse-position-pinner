/** 与 Rust 侧 Settings 结构保持一致的镜像类型（serde camelCase）。 */
export interface Settings {
  hotkey: string;
  alwaysOnTop: boolean;
  window: { x: number; y: number } | null;
}
