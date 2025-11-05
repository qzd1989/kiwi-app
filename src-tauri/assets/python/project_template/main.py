import time
from kiwi import (
    ColoredPoint,
    Key,
    Point,
    Response,
    RgbOffset,
    Client,
    System,
    WeightPoint,
    Size,
    RelativeColoredPoint,
)

client = Client()

while True:
    data = client.get_mouse_location().data
    print("location:", data, data.x, data.y)
    System.sleep(milliseconds=1000)
