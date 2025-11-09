import { u32 } from "./u32";

interface Release {
  signature: string;
  version: string;
  pub_date: string; // RFC 3339 格式的字符串 "2025-07-02T15:43:00+08:00"
  force_update: boolean;
  notes: string[];
  url: string;
  size: u32;
}

export type { Release };
