<script>
import Keycloak from 'keycloak-js';
import { onMount, setContext } from 'svelte';

let authPromise;
let keycloak;

setContext('authClient', { getAuthClient: () => keycloak });

onMount(() => {
	console.log('on mount');
	keycloak = new Keycloak();
    
 	document.querySelectorAll('iframe').forEach((elem) => {
 	    elem.parentNode.removeChild(elem);
 	});
    
	authPromise = keycloak.init({
		onLoad: 'check-sso',
		silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso'
	});
	console.log(authPromise);
});

function onLogin() {
	keycloak.login({ redirectUri: 'http://localhost:5173/callback' });
}
</script>

<div>
	{#if authPromise}
		{#await authPromise}
			<p>waiting...</p>
		{:then authenticated}
			{#if authenticated}
				<p style="color: green">AUTHENTICATED</p>
				<slot />
			{:else}
				Please <a href={'#'} on:click={onLogin}>sign in</a>
			{/if}
		{:catch error}
            {console.log(error)}
			<p style="color: red">Somfing wrong...</p>
		{/await}
	{/if}
</div>
