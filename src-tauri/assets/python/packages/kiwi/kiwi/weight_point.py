from __future__ import annotations
from dataclasses import dataclass
from .point import Point
from types import SimpleNamespace
from typing import Optional


@dataclass
class WeightPoint:
    point: Point
    weight: float

    def _to_dict(self) -> dict:
        return {"point": self.point, "weight": self.weight}

    @staticmethod
    def _from_namespace(ns: SimpleNamespace | None) -> Optional[WeightPoint]:
        if ns is None:
            return None
        return WeightPoint(point=Point._from_namespace(ns.point), weight=ns.weight)

    @staticmethod
    def _from_namespace_array(
        ns_array: list[SimpleNamespace] | None,
    ) -> list[WeightPoint]:
        if ns_array is None:
            return []
        return [WeightPoint._from_namespace(ns) for ns in ns_array if ns is not None]
