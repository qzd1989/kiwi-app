import time
from kiwi import (
    ColoredPoint,
    Key,
    Point,
    Response,
    RgbOffset,
    ScreenClient,
    System,
    WeightPoint,
    Size,
    RelativeColoredPoint,
)

client = ScreenClient()

while True:
    data = client.get_mouse_location().data
    print("location:", data, data.x, data.y)
    System.sleep(milliseconds=1000)
