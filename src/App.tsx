import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { formatDisplay } from './lib/coords';
import { useCursorPosition } from './hooks/useCursorPosition';
import SettingsPopup from './components/SettingsPopup';

interface CopyPayload {
  x: number;
  y: number;
  text: string;
}

export default function App() {
  const pos = useCursorPosition(30);
  const [copied, setCopied] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const flashTimer = useRef<number | undefined>(undefined);

  const flash = useCallback(() => {
    setCopied(true);
    window.clearTimeout(flashTimer.current);
    flashTimer.current = window.setTimeout(() => setCopied(false), 1200);
  }, []);

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;
    let cancelled = false;
    void listen<CopyPayload>('copy-done', () => flash()).then((fn) => {
      if (cancelled) fn();
      else unlisten = fn;
    });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [flash]);

  return (
    <div className="panel">
      <div className="controls">
        <button
          className="ctl"
          title="设置"
          onClick={() => setShowSettings((s) => !s)}
        >
          ⚙
        </button>
        <button className="ctl" title="退出" onClick={() => void invoke('exit_app')}>
          ×
        </button>
      </div>

      {showSettings ? (
        <SettingsPopup onClose={() => setShowSettings(false)} onError={setError} />
      ) : (
        <div className="coords">{formatDisplay(pos?.x ?? null, pos?.y ?? null)}</div>
      )}

      {copied && <div className="copy-flash">已复制 ✓</div>}
      {error && <div className="error-toast">{error}</div>}
    </div>
  );
}
