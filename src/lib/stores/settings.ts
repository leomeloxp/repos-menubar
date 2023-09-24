import { writable } from "svelte/store";
import { Store } from "tauri-plugin-store-api";

type UserSettings = {
  editor: 'vscode' | 'vscode-insiders'
}

const defaultSettings: UserSettings = {
  editor: 'vscode'
}

// @TODO: make store path an env variable
const store = new Store(".settings.json");
export const settings = writable<UserSettings>(defaultSettings, (set) => {
  store.get("user-settings").then((settings: UserSettings) => {
    if (settings) {
      set(settings);
    }
  });
});

settings.subscribe((value) => {
  store.set("user-settings", value).then(()=> {
    store.save();
  });
});
