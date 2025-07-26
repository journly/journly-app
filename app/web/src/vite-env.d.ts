/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string;
  readonly VITE_GOOGLE_OAUTH_CLIENT_ID: string;
  readonly VITE_GOOGLE_OAUTH_REDIRECT: string;
  readonly VITE_MAPBOX_ACCESS_TOKEN: string;
  readonly VITE_REPLICACHE_PUSH_URL: string;
  readonly VITE_REPLICACHE_PULL_URL: string;
  readonly VITE_REPLICACHE_POKE_URL: string;
  // add any other variables you use:
  // readonly VITE_SOME_KEY: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
