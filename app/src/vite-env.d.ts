/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_GITHUB_TAG: string
  readonly VITE_API_URL: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
