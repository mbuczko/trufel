import { readable, writable } from 'svelte/store';
import { dev } from '$app/environment';

/** @type import('svelte/store').Writable<String> */
export const notification = writable('');

export const apiHost = readable(dev ? 'http://localhost:8002' : '');

export const keycloak = writable({});

export const identity = writable({
	id: null,
	firstName: null,
	lastName: null,
	picture: null,
	authenticating: false,
	attributes: {
		picture: null
	}
});


