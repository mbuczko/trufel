<script>
import { onMount, setContext } from 'svelte';
import { goto } from '$app/navigation';
import Keycloak from 'keycloak-js';

onMount(async () => {
	const keycloak = new Keycloak('/keycloak.json');
	keycloak
		.init({ onLogin: 'check-sso' })
		.then(keycloak.loadUserProfile)
		.then((profile) => {
			let token = keycloak.token;
            let idToken = keycloak.idToken;
            console.log(token);
            console.log(idToken)
			fetch('http://localhost:3030/user', {
				method: 'POST',
				mode: 'cors',
				headers: {
					Origin: 'http://localhost:3000',
					Authorization: `Bearer ${idToken}`,
					'Content-type': 'application/json'
				}
			});
        })
		.then(() => goto('/authenticated/profile'));
});
</script>
