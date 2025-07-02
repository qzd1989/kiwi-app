type Locale = "en-US" | "zh-CN";
interface AppLocale {
  key: Locale;
  name: string;
}
const locales: AppLocale[] = [
  { key: "en-US", name: "English" },
  { key: "zh-CN", name: "简体中文" },
];
export { locales };
export type { Locale, AppLocale };
