import time
from kiwi_assistant import (
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
    point = client.get_mouse_location()
    print("point:", point, point.x, point.y)
    System.sleep(milliseconds=1000)
