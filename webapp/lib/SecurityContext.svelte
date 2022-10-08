<script>
import { onMount, setContext } from 'svelte';
import { identity } from '$lib/store.js';
import Keycloak from 'keycloak-js';

let authPromise;
let keycloak;

setContext('auth', { getAuthClient: () => keycloak });

onMount(() => {
	keycloak = new Keycloak('/keycloak.json');
	/* document.querySelectorAll('iframe').forEach((elem) => {
 	   elem.parentNode.removeChild(elem);
 	   }); */

	authPromise = keycloak
		.init({
			onLoad: 'check-sso',
			silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso'
		})
		.then((authenticated) => {
			return (authenticated && !$identity)
				 ? fetch('http://localhost:3030/@me', {
					 method: 'GET',
					 headers: {
						 Authorization: `Bearer ${keycloak.token}`,
						 Accept: 'application/json'
					 }
				 })
                 : Promise.resolve(authenticated);
		})
		.then((response) => {
			return response && response.ok ? response.json() : Promise.resolve(response);
		})
		.then((result) => {
			return typeof result == 'boolean'
				 ? Promise.resolve(result)
			     : new Promise((resolve, _) => {
					 identity.set(result);
					 resolve(true);
			     })
		});

	const nav = document.getElementById('top-nav');
	nav.addEventListener('login', (e) => {
		keycloak.login({ redirectUri: 'http://localhost:5173/callback' });
	});
	nav.addEventListener('logout', (e) => {
		keycloak.logout({ redirectUri: 'http://localhost:5173/'});
	});
});
</script>

<div>
	{#if authPromise}
		{#await authPromise}
			<p>waiting...</p>
		{:then authenticated}
			{#if authenticated}
				<slot />
			{:else}
				Dupa
			{/if}
		{:catch error}
			{console.log(error)}
			<p style="color: red">Somfing wrong...</p>
		{/await}
	{/if}
</div>
