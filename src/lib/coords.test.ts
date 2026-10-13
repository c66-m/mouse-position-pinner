import { describe, it, expect } from 'vitest';
import { formatCoords, formatDisplay } from './coords';

describe('formatCoords', () => {
  it('正坐标输出纯坐标文本', () => {
    expect(formatCoords(1234, 567)).toBe('1234, 567');
  });

  it('负坐标（多显示器副屏）正常输出', () => {
    expect(formatCoords(-1920, 300)).toBe('-1920, 300');
  });

  it('零坐标正常输出', () => {
    expect(formatCoords(0, 0)).toBe('0, 0');
  });
});

describe('formatDisplay', () => {
  it('渲染窗口展示文本', () => {
    expect(formatDisplay(1234, 567)).toBe('X: 1234  Y: 567');
  });

  it('无数据时显示占位', () => {
    expect(formatDisplay(null, null)).toBe('X: ---  Y: ---');
  });
});
