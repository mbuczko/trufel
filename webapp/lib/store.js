import { readable, writable } from 'svelte/store';
import { dev } from '$app/environment';

export const apiHost = readable(dev ? 'http://localhost:8002' : '');
export const identity = writable({});
