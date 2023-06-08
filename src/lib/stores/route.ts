import { writable } from "svelte/store";

export const route = writable<'main' | 'settings'>('main');

export function goToSettingsView() {
  route.set('settings');
}

export function goToMainView() {
  route.set('main');
}
