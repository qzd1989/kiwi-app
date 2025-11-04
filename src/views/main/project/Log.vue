<script setup lang="ts">
import { ref, nextTick } from "vue";
import { listen } from "@tauri-apps/api/event";
import { Stack } from "@types";
import { ElScrollbar } from "element-plus";

interface EmitLog {
  data: string;
  time: number;
}

class Log {
  constructor(
    public type: "info" | "warn" | "error" | "success",
    public message: string,
    public time: number,
    public formattedTime: string,
  ) {}

  static info(message: string, time: number): Log {
    return new Log("info", message, time, Log.formatTime(time));
  }

  static warn(message: string, time: number): Log {
    return new Log("warn", message, time, Log.formatTime(time));
  }

  static error(message: string, time: number): Log {
    return new Log("error", message, time, Log.formatTime(time));
  }

  static success(message: string, time: number): Log {
    return new Log("success", message, time, Log.formatTime(time));
  }

  static formatTime(timestamp: number) {
    const date = new Date(timestamp * 1000);
    const year = date.getFullYear().toString().slice(-2);
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    const seconds = (date.getSeconds() + (timestamp % 1))
      .toFixed(3)
      .padStart(6, "0");
    const formattedTime = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
    return formattedTime;
  }
}

const logScrollbarRef = ref<InstanceType<typeof ElScrollbar> | null>(null);
const logs = ref<Stack<Log>>(new Stack(100));

const logScrollToBottom = () => {
  nextTick(() => {
    if (!logScrollbarRef.value) return;
    setTimeout(() => {
      const wrap = logScrollbarRef.value?.wrapRef;
      if (wrap) {
        wrap.scrollTop = wrap.scrollHeight;
      }
    }, 10);
  });
  1;
};

const clear = () => {
  logs.value.clear();
};

listen<EmitLog>("project:log:info", (event) => {
  const log = Log.info(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitLog>("project:log:warn", (event) => {
  const log = Log.warn(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitLog>("project:log:error", (event) => {
  const log = Log.error(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitLog>("project:log:success", (event) => {
  const log = Log.success(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

defineExpose({
  clear,
});
</script>
<template>
  <el-scrollbar class="logs bg-green-900" ref="logScrollbarRef">
    <ul class="table px-2 py-2 text-sm">
      <li
        v-for="log in logs.stack"
        class="table-row w-full"
        :class="{
          'text-gray-400': log.type == 'info',
          'text-amber-400': log.type == 'warn',
          'text-red-400': log.type == 'error',
          'text-green-400': log.type == 'success',
        }"
      >
        <div class="table-cell py-0.5 pr-2 whitespace-nowrap">
          [{{ log.type.toUpperCase() }}]
        </div>
        <div class="table-cell py-0.5 pr-2 whitespace-nowrap">
          {{ log.formattedTime }}
        </div>
        <div class="table-cell py-0.5">{{ log.message }}</div>
      </li>
    </ul>
  </el-scrollbar>
</template>
<style scoped></style>
