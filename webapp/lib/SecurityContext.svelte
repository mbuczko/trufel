<script>
import { onMount, setContext } from 'svelte';
import { identity } from '$lib/store.js';
import Keycloak from 'keycloak-js';
import Spinner from './Spinner.svelte';

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
			return (authenticated && (!$identity || !$identity.user_id))
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
			return result === false
		         ? new Promise((resolve, _) => {
                     identity.set(null);
                     resolve(false);
                 })
			     : new Promise((resolve, _) => {
                     // result = true means that user is authenticated
                     // and its identity has been already saved in a store.
                     if (result !== true) {
                         identity.set(result);
                     }
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
				<p>Unauthenticated</p>
			{/if}
		{:catch error}
			{console.log(error)}
			<p style="color: red">Somfing wrong...</p>
		{/await}
	{/if}
</div>
