import { Language } from "@types";

interface EmitProject {
  name: string | null;
  language: Language;
  mainFile: string | null;
  path: string | null;
  kiwiVersion: string | null;
  mainFileFullPath?: string;
}

namespace EmitProject {
  export const empty = (): EmitProject => ({
    name: null,
    language: "python",
    mainFile: null,
    path: null,
    kiwiVersion: null,
  });
}

export { EmitProject };
