from __future__ import annotations
from dataclasses import dataclass
from .point import Point
from types import SimpleNamespace
from typing import Optional


@dataclass
class ColoredPoint:
    point: Point
    hex: str

    def _to_dict(self) -> dict:
        return {
            "point": self.point._to_dict(),
            "hex": self.hex,
        }

    @staticmethod
    def _from_namespace(ns: SimpleNamespace | None) -> Optional[ColoredPoint]:
        if ns is None:
            return None
        return ColoredPoint(point=Point._from_namespace(ns.point), hex=ns.hex)

    @staticmethod
    def _from_namespace_array(
        ns_array: list[SimpleNamespace] | None,
    ) -> list[ColoredPoint]:
        if ns_array is None:
            return []
        return [ColoredPoint._from_namespace(ns) for ns in ns_array if ns is not None]
