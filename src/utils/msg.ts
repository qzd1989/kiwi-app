import { ElMessage } from "element-plus";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

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

const msgError = (e: unknown, duration?: number) => {
  let message: string;

  if (e instanceof Error) {
    message = e.message;
  } else if (typeof e === "object" && e !== null) {
    if ("message" in e) {
      message = e.message as string;
    } else {
      try {
        message = JSON.stringify(e, null, 2);
      } catch {
        message = t("Can't parse error object.");
      }
    }
  } else {
    message = String(e);
  }

  if (duration) {
    ElMessage({
      showClose: true,
      message: `${message}`,
      type: "error",
      grouping: true,
      duration,
    });
  } else {
    ElMessage({
      showClose: true,
      message: `${message}`,
      type: "error",
      grouping: true,
      duration: 0,
    });
  }
};

export { msgError, msgSuccess, msgWarn, msgInfo };
