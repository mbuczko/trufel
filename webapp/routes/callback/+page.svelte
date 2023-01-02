<script>
import { onMount, setContext } from 'svelte';
import { goto } from '$app/navigation';
import { identity } from '$lib/store.js';
import Keycloak from 'keycloak-js';

onMount(async () => {
	const keycloak = new Keycloak('/keycloak.json');
	keycloak
		.init({ onLogin: 'check-sso' })
		.then(keycloak.loadUserProfile)
		.then((profile) => {
			return fetch('http://localhost:3030/user', {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${keycloak.idToken}`,
					Accept: 'application/json',
					'Content-type': 'application/json'
				}
			});
		})
		.then((response) => {
			if (response.ok) {
				return response.json();
			} else {
                console.error("Could fetch user's identity");
            }
		})
		.then((ident) => {
			identity.set(ident);
			goto('/authenticated/profile');
		});
});
</script>
