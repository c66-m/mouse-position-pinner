import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import type { Settings } from '../lib/settings';

const EXPANDED = new LogicalSize(230, 150);
const COLLAPSED = new LogicalSize(190, 64);

interface Props {
  onClose: () => void;
  onError: (msg: string | null) => void;
}

export default function SettingsPopup({ onClose, onError }: Props) {
  const [hotkey, setHotkey] = useState('');
  const [alwaysOnTop, setAlwaysOnTop] = useState(true);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    const win = getCurrentWindow();
    void win.setSize(EXPANDED);
    let cancelled = false;
    void invoke<Settings>('get_settings')
      .then((s) => {
        if (cancelled) return;
        setHotkey(s.hotkey);
        setAlwaysOnTop(s.alwaysOnTop);
      })
      .catch((e) => onError(String(e)));
    return () => {
      cancelled = true;
      void win.setSize(COLLAPSED);
    };
  }, [onError]);

  const save = async () => {
    setBusy(true);
    try {
      await invoke('update_settings', { patch: { hotkey, alwaysOnTop } });
      onClose();
    } catch (e) {
      onError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="settings">
      <label className="row">
        <span className="lbl">热键</span>
        <input
          className="hk"
          value={hotkey}
          placeholder="如 F2 或 Ctrl+Shift+A"
          spellCheck={false}
          onChange={(e) => setHotkey(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') void save();
          }}
        />
      </label>

      <label className="row">
        <span className="lbl">置顶</span>
        <input
          type="checkbox"
          className="pin"
          checked={alwaysOnTop}
          onChange={(e) => setAlwaysOnTop(e.target.checked)}
        />
      </label>

      <div className="row btns">
        <button className="btn" onClick={() => void save()} disabled={busy}>
          完成
        </button>
        <button className="btn" onClick={onClose} disabled={busy}>
          取消
        </button>
      </div>

      <div className="hint">支持 F1–F24、字母、数字及 Ctrl / Alt / Shift / Win 组合</div>
    </div>
  );
}
