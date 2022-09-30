<script>
export const prerender = true;

import auth from '$lib/authService';
import { onDestroy, onMount } from 'svelte';
import { apiHost } from '$lib/store';

let authClient;

onMount(() => {
    console.log("on mount");
    authClient = auth.createClient();
});

onDestroy(() => {
    console.log("on destroy");
});

function onLogin() {
    if (authClient) {
        authClient.login({redirectUri: 'http://localhost:5173/callback'});
    }
}

</script>

<svelte:head>
	<title>Trufel</title>
</svelte:head>

<section>
	<div class="flex header bg-slate-500 shadow-lg p-2">
		<div class="mx-6 w-60 align-baseline mt-1">
			<span class="uppercase text-gray-100 text-xl font-semibold font-mono ml-9" style="text-shadow: 1px 1px #333">
                Trufel ]:>
            </span>
            <a href={"#"} on:click={onLogin}>Login</a>
		</div>
	</div>
</section>
