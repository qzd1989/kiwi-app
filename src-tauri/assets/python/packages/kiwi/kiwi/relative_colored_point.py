from __future__ import annotations
from dataclasses import dataclass
from .point import Point
from types import SimpleNamespace
from typing import Optional
from .colored_point import ColoredPoint


@dataclass
class RelativeColoredPoint:
    colored_point: ColoredPoint
    relative_point: Point

    def _to_dict(self) -> dict:
        return {
            "colored_point": self.colored_point._to_dict(),
            "relative_point": self.relative_point._to_dict(),
        }

    @staticmethod
    def _from_namespace(ns: SimpleNamespace | None) -> Optional[RelativeColoredPoint]:
        if ns is None:
            return None
        return RelativeColoredPoint(
            colored_point=ColoredPoint._from_namespace(ns.colored_point),
            relative_point=Point._from_namespace(ns.relative_point),
        )

    @staticmethod
    def _from_namespace_array(
        ns_array: list[SimpleNamespace] | None,
    ) -> list[RelativeColoredPoint]:
        if ns_array is None:
            return []
        return [
            RelativeColoredPoint._from_namespace(ns)
            for ns in ns_array
            if ns is not None
        ]
