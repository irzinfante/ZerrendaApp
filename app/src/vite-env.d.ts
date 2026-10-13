/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_GITHUB_TAG: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
