# 你可以根据需要修改或删除该文件。
# You may modify or delete this file as needed.

from typing import Optional
from kiwi_assistant import (
    Point,
    Client,
    System,
)

Method = str

client = Client()


def move_and_click(point: Point):
    client.move_absolute_smooth(point)
    System.sleep(10)
    client.click_left()


def abs_move(point: Point):
    client.move_absolute_smooth(point)


def sleep(s: float):
    System.sleep(round(s * 1000))


def sleep_ms(ms: int):
    System.sleep(ms)


# 这是一个带一次重试机制的简易任务流类。当节点执行失败时会中断，下次运行会对该节点重试一次；若仍然失败，将回退到上一个节点继续执行。可与 System.tick 配合使用。
# This is a lightweight task-flow class with a single retry mechanism. When a node fails, the flow is interrupted; on the next run, the node will be retried once. If it fails again, the flow will roll back to the previous node and continue from there. It can be used in conjunction with System.tick.
#
# 示例/example:
# 如果 example_task1 的某个节点执行失败，流程会停留在该节点并继续执行 example_task2。待 example_task2 完成后，流程将回到 example_task1 的失败节点继续执行；如果再次失败，则回退到上一个节点，等待下次继续执行。
# If a node in example_task1 fails, the flow will pause at that node and continue with example_task2. Once example_task2 completes, the flow will return to the failed node in example_task1 to continue execution; if it fails again, it will roll back to the previous node and wait for the next run to continue.
#
# class ExampleTaskFlow(TaskFlow):
#      def __init__(self):
#         self.entrance = "node_one"  #执行入口,必须设置./entry point, must be set.
#         self.reset() #状态初始化,必须设置/state initialization, must be set.
#     def node_one(self) -> Optional[Method]:
#         print("Executing node_one")
#         move_and_click(Point(100, 100))
#         return "node_two"

#     def node_two(self) -> Optional[Method]:
#         print("Executing node_two")
#         move_and_click(Point(200, 200))
#         return False

#     def node_three(self) -> Optional[Method]:
#         print("Executing step_three")
#         move_and_click(Point(300, 300))
#         return True # 最后一个节点执行成功的话必须返回真/last node must return true if executed successfully
#
# example_task1 = ExampleTaskFlow()
# example_task2 = ExampleTaskFlow()
#
# while True:
#     System.tick(example_task1.run(), 10000) # 每10秒执行一次example_task1/Execute example_task1 every 10 seconds.
#     System.tick(example_task2.run(), 20000) # 每20秒执行一次example_task2/Execute example_task1 every 20 seconds.
#     System.sleep(100)


class TaskFlow:
    entrance: Method
    stack: list[str]
    is_interrupted: bool

    def __init__(self, entrance: Method):
        self.entrance = entrance
        self.reset()

    def reset(self):
        self.is_interrupted = False
        self.stack = []
        self.append(self.entrance)

    def len(self) -> int:
        return len(self.stack)

    def append(self, method: Method):
        self.stack.append(method)

    def pop(self) -> Optional[Method]:
        if self.len() == 0:
            return None
        else:
            return self.stack.pop()

    def last(self) -> Optional[Method]:
        if self.len() == 0:
            return None
        else:
            return self.stack[-1]

    def interrupt(self):
        self.is_interrupted = True

    def resume(self):
        self.is_interrupted = False

    def done(self):
        print(f"{self.__class__.__name__}.done")

    def run(self):
        method_name = self.last()
        method = getattr(self, method_name)
        print(f"{self.__class__.__name__}.{method_name}")
        result = method()

        if result is True:
            self.done()
            self.reset()
        elif result is False:
            if self.is_interrupted:
                self.pop()
                if self.len() == 0:
                    self.reset()
                self.run()
            else:
                self.interrupt()
                print(f"{self.__class__.__name__}.{method_name} 被打断")
        else:
            self.append(result)
            self.run()
