from __future__ import annotations
from dataclasses import dataclass
from types import SimpleNamespace
from typing import Optional


@dataclass
class RgbOffset:
    r: int
    g: int
    b: int

    def _to_dict(self) -> dict:
        return {"r": self.r, "g": self.g, "b": self.b}

    @staticmethod
    def _from_namespace(ns: SimpleNamespace | None) -> Optional[RgbOffset]:
        if ns is None:
            return None
        return RgbOffset(r=ns.r, g=ns.g, b=ns.b)
