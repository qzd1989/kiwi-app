import argparse
from websocket import create_connection
from types import SimpleNamespace
from typing import Any
from typing import Optional, cast, Union
import atexit
import json
import sys
from .point import Point
from .colored_point import ColoredPoint
from .relative_colored_point import RelativeColoredPoint
from .response import (
    BoolResponse,
    PointResponse,
    Response,
    ColoredPointResponse,
    ColoredPointsResponse,
    StrResponse,
    WeightPointResponse,
    WeightPointsResponse,
)
from .rgb_offset import RgbOffset
from .key import Key
from .system import System
from .weight_point import WeightPoint


class Client:
    def __init__(self):
        self.ws = None
        self._connect()
        atexit.register(self._close)

    def __del__(self):
        self._close()

    def _connect(self):
        parser = argparse.ArgumentParser()
        parser.add_argument("--endpoint", type=str, default="127.0.0.1:9927")
        args, _ = parser.parse_known_args()
        self.ws = create_connection(f"ws://{args.endpoint}/")

    def _close(self):
        try:
            if self.ws is None:
                return
            self.ws.close()
            self.ws = None
        except Exception as e:
            print("Error on close:", e)

    def _send_and_receive(self, method: str, args: dict) -> Optional[Response]:
        if self.ws is None:
            self._connect()
        data = {"method": method, "args": args}
        try:
            self.ws.send(json.dumps(data))
            result = self.ws.recv()
        except Exception:
            self._connect()
            self.ws.send(json.dumps(data))
            result = self.ws.recv()
        return self._parse_response(result)

    def _dict_to_namespace(self, obj: Any) -> Any:
        if isinstance(obj, dict):
            return SimpleNamespace(
                **{k: self._dict_to_namespace(v) for k, v in obj.items()}
            )
        elif isinstance(obj, list):
            return [self._dict_to_namespace(item) for item in obj]
        else:
            return obj

    def _parse_response(self, json_str) -> Optional[Response]:
        if not json_str.strip():
            return None
        try:
            json_data = json.loads(json_str)
        except json.JSONDecodeError as e:
            print(f"JSON decode error: {e}")
            return None
        if not (
            "status" in json_data and "message" in json_data and "data" in json_data
        ):
            print("Response missing required keys")
            return None
        return Response(
            status=json_data["status"],
            message=json_data.get("message"),
            data=self._dict_to_namespace(json_data.get("data")),
        )

    def find_image(
        self,
        subpath: str,
        start_point: Point,
        end_point: Point,
        threshold: float,
        min_template_side: int,
    ) -> Optional[WeightPointResponse]:
        args = {
            "subpath": subpath,
            "start_point": start_point._to_dict(),
            "end_point": end_point._to_dict(),
            "threshold": threshold,
            "min_template_side": min_template_side,
        }
        response = self._send_and_receive("find_image", args)
        response.data = WeightPoint._from_namespace(response.data)
        return WeightPointResponse._from(response=response)

    def find_images(
        self,
        subpath: str,
        start_point: Point,
        end_point: Point,
        threshold: float,
        min_template_side: int,
    ) -> Optional[WeightPointsResponse]:
        args = {
            "subpath": subpath,
            "start_point": start_point._to_dict(),
            "end_point": end_point._to_dict(),
            "threshold": threshold,
            "min_template_side": min_template_side,
        }
        response = self._send_and_receive("find_images", args)
        response.data = WeightPoint._from_namespace_array(response.data)
        return WeightPointsResponse._from(response=response)

    def find_relative_colors(
        self,
        points: list[RelativeColoredPoint],
        start_point: Point,
        end_point: Point,
        rgb_offset: RgbOffset,
    ) -> Optional[ColoredPointResponse]:
        args = {
            "points": [rp._to_dict() for rp in points],
            "start_point": start_point._to_dict(),
            "end_point": end_point._to_dict(),
            "rgb_offset": rgb_offset._to_dict(),
        }
        response = self._send_and_receive("find_relative_colors", args)
        response.data = ColoredPoint._from_namespace(response.data)
        return ColoredPointResponse._from(response=response)

    def find_colors(
        self,
        hex_colors: list[str],
        start_point: Point,
        end_point: Point,
        rgb_offset: RgbOffset,
    ) -> Optional[ColoredPointsResponse]:
        args = {
            "hex_colors": hex_colors,
            "start_point": start_point._to_dict(),
            "end_point": end_point._to_dict(),
            "rgb_offset": rgb_offset._to_dict(),
        }
        response = self._send_and_receive("find_colors", args)
        response.data = ColoredPoint._from_namespace_array(response.data)
        return ColoredPointsResponse._from(response=response)

    def recognize_text(
        self, start_point: Point, end_point: Point
    ) -> Optional[StrResponse]:
        args = {
            "start_point": start_point._to_dict(),
            "end_point": end_point._to_dict(),
        }
        response = self._send_and_receive("recognize_text", args)
        return StrResponse._from(response=response)

    def save_frame(self, path: str) -> Optional[BoolResponse]:
        args = {
            "path": path,
        }
        response = self._send_and_receive("save_frame", args)
        return BoolResponse._from(response=response)

    def get_mouse_location(self) -> Optional[PointResponse]:
        args = {}
        response = self._send_and_receive("get_mouse_location", args)
        response.data = Point._from_namespace(response.data)
        return PointResponse._from(response=response)

    def click_left(self):
        args = {}
        self._send_and_receive("click_left", args)

    def click_right(self):
        args = {}
        self._send_and_receive("click_right", args)

    def press_left(self):
        args = {}
        self._send_and_receive("press_left", args)

    def press_right(self):
        args = {}
        self._send_and_receive("press_right", args)

    def release_left(self):
        args = {}
        self._send_and_receive("release_left", args)

    def release_right(self):
        args = {}
        self._send_and_receive("release_right", args)

    def move_absolute(self, x: int, y: int):
        absolute_point = Point(x, y)
        args = {"absolute_point": absolute_point._to_dict()}
        self._send_and_receive("move_absolute", args)

    def move_relative(self, x: int, y: int):
        offset = Point(x, y)
        args = {"offset": offset._to_dict()}
        self._send_and_receive("move_relative", args)

    def scroll_vertical(self, length: int):
        args = {"length": length}
        self._send_and_receive("scroll_vertical", args)

    def scroll_horizontal(self, length: int):
        args = {"length": length}
        self._send_and_receive("scroll_horizontal", args)

    def press_key(self, key: Key):
        key_val = key.value if isinstance(key, Key) else key
        args = {"key": key_val}
        self._send_and_receive("press_key", args)

    def release_key(self, key: Key):
        key_val = key.value if isinstance(key, Key) else key
        args = {"key": key_val}
        self._send_and_receive("release_key", args)

    def click_key(self, key: Key):
        key_val = key.value if isinstance(key, Key) else key
        args = {"key": key_val}
        self._send_and_receive("click_key", args)

    def copy(self):
        system = System()
        if sys.platform == "darwin":
            self.press_key(key=Key.Meta)
            system.sleep(milliseconds=20)
            self.click_key(key="c")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Meta)
        else:
            self.press_key(key=Key.Control)
            system.sleep(milliseconds=20)
            self.click_key(key="c")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Control)

    def paste(self):
        system = System()
        if sys.platform == "darwin":
            self.press_key(key=Key.Meta)
            system.sleep(milliseconds=20)
            self.click_key(key="v")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Meta)
        else:
            self.press_key(key=Key.Control)
            system.sleep(milliseconds=20)
            self.click_key(key="v")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Control)

    def cut(self):
        system = System()
        if sys.platform == "darwin":
            self.press_key(key=Key.Meta)
            system.sleep(milliseconds=20)
            self.click_key(key="x")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Meta)
        else:
            self.press_key(key=Key.Control)
            system.sleep(milliseconds=20)
            self.click_key(key="x")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Control)

    def select_all(self):
        system = System()
        if sys.platform == "darwin":
            self.press_key(key=Key.Meta)
            system.sleep(milliseconds=20)
            self.click_key(key="a")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Meta)
        else:
            self.press_key(key=Key.Control)
            system.sleep(milliseconds=20)
            self.click_key(key="a")
            system.sleep(milliseconds=20)
            self.release_key(key=Key.Control)

    def text(self, text: str):
        args = {"text": text}
        self._send_and_receive("input_text", args)
