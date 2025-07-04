import time
from kiwi import ScreenClient, Point, RelativePoint, RgbOffset, System, Key

client = ScreenClient()

while True:
    data = client.get_mouse_location().data
    print("location:", data, data.x, data.y)
    System.sleep(milliseconds=1000)
