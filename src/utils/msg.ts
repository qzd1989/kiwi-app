import { ElMessage } from "element-plus";

const msgErrorObject = (e: Error, duration?: number) => {
  msgError(e.message, duration);
};

const msgInfo = (msg: string) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "info",
    message: `${msg}`,
  });
};

const msgWarn = (msg: string) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "warning",
    message: `${msg}`,
  });
};

const msgSuccess = (msg: string) => {
  ElMessage({
    showClose: true,
    message: `${msg}`,
    type: "success",
    grouping: true,
  });
};

const msgError = (msg: string, duration?: number) => {
  if (duration) {
    ElMessage({
      showClose: true,
      message: `${msg}`,
      type: "error",
      grouping: true,
      duration,
    });
  } else {
    ElMessage({
      showClose: true,
      message: `${msg}`,
      type: "error",
      grouping: true,
      duration: 0,
    });
  }
};

export { msgError, msgErrorObject, msgSuccess, msgWarn, msgInfo };
