import { useLocalStore } from "@store";
class Locale {
  static defaultLocale(): string {
    return "en-US";
  }

  static all() {
    return [
      { value: "en-US", label: "English" },
      { value: "zh-CN", label: "中文" },
    ];
  }

  static async get(): Promise<string> {
    const localStore = useLocalStore();
    return (await localStore.get<string>("locale")) ?? Locale.defaultLocale();
  }

  static set(locale: string) {
    const localStore = useLocalStore();
    localStore.set("locale", locale);
  }
}

export { Locale };
