/**
 * 坐标格式化工具（与 Rust 侧 cursor::format_coords 口径一致）。
 */

export function formatCoords(x: number, y: number): string {
  return `${x}, ${y}`;
}

export function formatDisplay(x: number | null, y: number | null): string {
  if (x === null || y === null) {
    return 'X: ---  Y: ---';
  }
  return `X: ${x}  Y: ${y}`;
}
