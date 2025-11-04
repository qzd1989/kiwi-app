from __future__ import annotations
from dataclasses import dataclass
from types import SimpleNamespace
from typing import Optional


@dataclass
class Point:
    x: int
    y: int

    def _to_dict(self) -> dict:
        return {"x": self.x, "y": self.y}

    @staticmethod
    def _from_namespace(ns: SimpleNamespace | None) -> Optional[Point]:
        if ns is None:
            return None
        return Point(x=ns.x, y=ns.y)
