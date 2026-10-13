#!/usr/bin/env python3
"""生成 鼠标位置定位器 应用图标（1024x1024 RGBA PNG，仅用标准库）。

设计：深色圆角面板 + 绿色十字准星 + 四角取景框小刻度，呼应"坐标"主题。
"""
import struct
import zlib

S = 1024
R = 176
BG = (13, 17, 23)        # #0d1117
BORD = (46, 160, 67)     # #2ea043 外圈
LINE = (63, 185, 80)     # #3fb950 十字线
CORNER = (63, 185, 80)   # 四角刻度

C = S // 2
BAR = 26          # 十字线半宽
GAP = 78          # 中心缺口（半长）
BORDER_W = 40     # 外圈厚度


def inside_rounded(x: int, y: int, r: int) -> bool:
    if x < r and y < r:
        return (x - r) ** 2 + (y - r) ** 2 <= r * r
    if x < r and y >= S - r:
        return (x - r) ** 2 + (y - (S - 1 - r)) ** 2 <= r * r
    if x >= S - r and y < r:
        return (x - (S - 1 - r)) ** 2 + (y - r) ** 2 <= r * r
    if x >= S - r and y >= S - r:
        return (x - (S - 1 - r)) ** 2 + (y - (S - 1 - r)) ** 2 <= r * r
    return True


def corner_tick(x: int, y: int) -> bool:
    o = 56
    e = 150
    t = 9
    corners = [
        ((o <= x <= e) and (abs(y - o) <= t)) or ((abs(x - o) <= t) and (o <= y <= e)),
        ((S - 1 - e <= x <= S - 1 - o) and (abs(y - o) <= t)) or ((abs(x - (S - 1 - o)) <= t) and (o <= y <= e)),
        ((o <= x <= e) and (abs(y - (S - 1 - o)) <= t)) or ((abs(x - o) <= t) and (S - 1 - e <= y <= S - 1 - o)),
        ((S - 1 - e <= x <= S - 1 - o) and (abs(y - (S - 1 - o)) <= t)) or ((abs(x - (S - 1 - o)) <= t) and (S - 1 - e <= y <= S - 1 - o)),
    ]
    return any(corners)


def crosshair(x: int, y: int) -> bool:
    v = abs(x - C) <= BAR and (y < C - GAP or y > C + GAP)
    h = abs(y - C) <= BAR and (x < C - GAP or x > C + GAP)
    return v or h


def pixel(x: int, y: int):
    if not inside_rounded(x, y, R):
        return (0, 0, 0, 0)
    if not inside_rounded(x, y, R - BORDER_W):
        if crosshair(x, y):
            return (*LINE, 255)
        if corner_tick(x, y):
            return (*CORNER, 255)
        return (*BG, 255)
    return (*BORD, 255)


def encode_png(rows) -> bytes:
    def chunk(tag: bytes, data: bytes) -> bytes:
        c = struct.pack(">I", len(data)) + tag + data
        return c + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)

    raw = b"".join(b"\x00" + bytes(row) for row in rows)
    ihdr = struct.pack(">IIBBBBB", S, S, 8, 6, 0, 0, 0)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(raw, 9))
        + chunk(b"IEND", b"")
    )


def main():
    rows = []
    for y in range(S):
        row = []
        for x in range(S):
            row.extend(pixel(x, y))
        rows.append(row)
    with open("app-icon.png", "wb") as f:
        f.write(encode_png(rows))
    print("app-icon.png written (1024x1024)")


if __name__ == "__main__":
    main()
