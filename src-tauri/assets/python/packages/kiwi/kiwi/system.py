import time
from typing import Callable, Optional, Union


class System:
    @staticmethod
    def sleep(milliseconds: int):
        """暂停指定毫秒"""
        if milliseconds > 0:
            time.sleep(milliseconds / 1000.0)

    @staticmethod
    def timer_tick(interval_ms: int, *args, immediate: bool = True):
        """
        每隔 interval_ms 毫秒执行一次 callback。
        支持两种调用方式：
        1. timer_tick(interval_ms, callback)
        2. timer_tick(interval_ms, key, callback)

        immediate: 第一次调用是否立即执行，默认为 True
        """
        if not hasattr(System, "_last_times"):
            System._last_times = {}

        if len(args) == 1:
            key = None
            callback = args[0]
        elif len(args) == 2:
            key, callback = args
        else:
            raise ValueError(
                "timer_tick expects interval_ms + callback, or interval_ms + key + callback"
            )

        if not callable(callback):
            raise TypeError("callback must be callable")

        unique_key = key or str(id(callback))
        now = time.time() * 1000  # 毫秒
        last_time = System._last_times.get(unique_key, None)

        if last_time is None:
            if immediate:
                callback()
            System._last_times[unique_key] = now
            return

        if now - last_time >= interval_ms:
            callback()
            System._last_times[unique_key] = now
