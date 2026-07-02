import { invoke } from "@tauri-apps/api/core";
import { getDemoResponse } from "./demoData";

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

export class ServiceCommandError extends Error {
  constructor(
    public serviceName: string,
    public command: string,
    public cause?: unknown,
  ) {
    super(`${serviceName} command '${command}' failed`);
    this.name = "ServiceCommandError";
  }
}

export function isTauriRuntime() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);
}

export async function safeInvoke<T>(
  serviceName: string,
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isTauriRuntime()) {
    const demoResponse = getDemoResponse<T>(cmd, args);
    if (demoResponse !== undefined) {
      return Promise.resolve(demoResponse);
    }
    throw new ServiceCommandError(serviceName, cmd, "No browser/demo implementation exists");
  }

  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new ServiceCommandError(serviceName, cmd, err);
  }
}
