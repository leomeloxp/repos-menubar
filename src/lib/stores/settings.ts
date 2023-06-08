import { writable } from "svelte/store";

type UserSettings = {
  editor: 'vscode' | 'vscode-insiders'
}

const defaultSettings: UserSettings = {
  editor: 'vscode'
}

export const settings = writable<UserSettings>(defaultSettings, (set) => {
  const storedSettings = localStorage.getItem("user-settings");
  if (storedSettings) {
    set(JSON.parse(storedSettings));
  }
});

settings.subscribe((value) => {
  localStorage.setItem("user-settings", JSON.stringify(value));
});
