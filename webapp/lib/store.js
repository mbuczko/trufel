import { readable } from 'svelte/store';
import { dev } from '$app/environment';

export const apiHost = readable(dev ? 'http://localhost:8002' : '');
