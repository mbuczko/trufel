import { readable, writable } from 'svelte/store';
import { dev } from '$app/environment';

export const apiHost = readable(dev ? 'http://localhost:8002' : '');
export const identity = writable({
	id: null,
	firstName: null,
	lastName: null,
	picture: null,
	authenticating: false
});
export const keycloak = writable({});
