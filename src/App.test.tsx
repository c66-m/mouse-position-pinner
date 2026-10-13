import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import App from './App';

const mocks = vi.hoisted(() => {
  const invokeMock = vi.fn();
  const listeners = new Map<string, (payload: unknown) => void>();
  return { invokeMock, listeners };
});

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => mocks.invokeMock(...args),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: (event: string, handler: (e: any) => void) => {
    mocks.listeners.set(event, handler);
    return Promise.resolve(() => {
      mocks.listeners.delete(event);
    });
  },
}));

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({ setSize: vi.fn().mockResolvedValue(undefined) }),
  LogicalSize: class {
    constructor(public width: number, public height: number) {}
  },
}));

beforeEach(() => {
  mocks.invokeMock.mockReset();
  mocks.listeners.clear();
  mocks.invokeMock.mockImplementation((cmd: string) => {
    if (cmd === 'get_cursor_pos') return Promise.resolve({ x: 1234, y: 567 });
    if (cmd === 'get_settings')
      return Promise.resolve({ hotkey: 'F2', alwaysOnTop: true, window: null });
    return Promise.resolve(undefined);
  });
});

describe('鼠标位置定位器 App', () => {
  it('渲染实时坐标（来自 get_cursor_pos）', async () => {
    render(<App />);
    expect(await screen.findByText(/X: 1234\s+Y: 567/)).toBeInTheDocument();
  });

  it('收到 copy-done 事件后短暂显示「已复制 ✓」', async () => {
    render(<App />);
    await screen.findByText(/X: 1234\s+Y: 567/);

    const handler = mocks.listeners.get('copy-done');
    expect(handler).toBeDefined();
    handler!({ payload: { x: 1234, y: 567, text: '1234, 567' } });

    expect(await screen.findByText('已复制 ✓')).toBeInTheDocument();
  });

  it('点击 ⚙ 打开设置，可修改热键并保存', async () => {
    render(<App />);
    await screen.findByText(/X: 1234\s+Y: 567/);

    fireEvent.click(screen.getByTitle('设置'));
    const input = await screen.findByPlaceholderText('如 F2 或 Ctrl+Shift+A');
    expect(input).toHaveValue('F2');

    fireEvent.change(input, { target: { value: 'Ctrl+Shift+C' } });
    fireEvent.click(screen.getByText('完成'));

    await waitFor(() => {
      expect(mocks.invokeMock).toHaveBeenCalledWith('update_settings', {
        patch: { hotkey: 'Ctrl+Shift+C', alwaysOnTop: true },
      });
    });
  });

  it('点击 × 退出应用', async () => {
    render(<App />);
    fireEvent.click(screen.getByTitle('退出'));
    expect(mocks.invokeMock).toHaveBeenCalledWith('exit_app');
  });
});
