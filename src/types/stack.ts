class Stack<T> {
  limit: number;
  stack: T[];

  constructor(limit: number) {
    this.stack = [];
    this.limit = limit;
  }

  push(element: T) {
    if (this.stack.length >= this.limit) {
      this.stack.shift();
    }
    this.stack.push(element);
  }

  clear(): void {
    this.stack = [];
  }
}

export { Stack };
