<script>
import { identity } from '$lib/store.js';
</script>

<svelte:head>
	<title>Trufel</title>
</svelte:head>

<div>
	{#if $identity.authenticating}
		<p>waiting...</p>
	{:else if $identity.id}
		<slot />
	{:else}
		<p>Unauthenticated</p>
	{/if}
</div>
