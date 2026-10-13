import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface CursorPos {
  x: number;
  y: number;
}

export function useCursorPosition(refreshMs = 30): CursorPos | null {
  const [pos, setPos] = useState<CursorPos | null>(null);

  useEffect(() => {
    let cancelled = false;
    let inFlight = false;

    const tick = async () => {
      if (inFlight) return;
      inFlight = true;
      try {
        const p = await invoke<CursorPos>('get_cursor_pos');
        if (!cancelled) setPos(p);
      } catch {
      } finally {
        inFlight = false;
      }
    };

    void tick();
    const id = window.setInterval(() => void tick(), refreshMs);
    return () => {
      cancelled = true;
      window.clearInterval(id);
    };
  }, [refreshMs]);

  return pos;
}
