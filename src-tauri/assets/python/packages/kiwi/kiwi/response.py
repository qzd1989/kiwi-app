from dataclasses import dataclass
from typing import Literal, Any, Optional

from .point import Point
from .colored_point import ColoredPoint
from .rgb_offset import RgbOffset
from .weight_point import WeightPoint


@dataclass
class Response:
    status: Literal["success", "error"]
    message: Optional[str]
    data: Optional[Any]

    def _to_dict(self) -> dict:
        return {
            "status": self.status,
            "message": self.message,
            "data": self.data,
        }


@dataclass
class WeightPointResponse(Response):
    data: Optional[WeightPoint]

    @staticmethod
    def _from(response: Response) -> "WeightPointResponse":
        return WeightPointResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class WeightPointsResponse(Response):
    data: Optional[list[WeightPoint]]

    @staticmethod
    def _from(response: Response) -> "WeightPointsResponse":
        return WeightPointsResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class ColoredPointResponse(Response):
    data: Optional[ColoredPoint]

    @staticmethod
    def _from(response: Response) -> "ColoredPointResponse":
        return ColoredPointResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class ColoredPointsResponse(Response):
    data: Optional[list[ColoredPoint]]

    @staticmethod
    def _from(response: Response) -> "ColoredPointsResponse":
        return ColoredPointsResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class StrResponse(Response):
    data: Optional[str]

    @staticmethod
    def _from(response: Response) -> "StrResponse":
        return StrResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class BoolResponse(Response):
    data: Optional[bool]

    @staticmethod
    def _from(response: Response) -> "BoolResponse":
        return BoolResponse(
            status=response.status, message=response.message, data=response.data
        )


@dataclass
class PointResponse(Response):
    data: Optional[Point]

    @staticmethod
    def _from(response: Response) -> "PointResponse":
        return PointResponse(
            status=response.status, message=response.message, data=response.data
        )
