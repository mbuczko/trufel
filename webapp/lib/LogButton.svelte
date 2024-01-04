<script>
import { identity } from '$lib/store.js';
import Spinner from '$lib/Spinner.svelte';
import Login from 'svelte-material-icons/Login.svelte';
import Logout from 'svelte-material-icons/Logout.svelte';
import { getContext } from 'svelte';

const defaultPicture = '/avatar.jpg';
const { login, logout } = getContext('auth');
</script>

<span>
	{#if $identity.authenticating}
		<Spinner />
	{:else if $identity.id}
		<span class="p-1 text-sky-200">
			{$identity.firstName}
		</span>
		|
		<span class="p-1 pr-3 text-slate-400 inline-block align-bottom">
			<a class="flex" href={'#'} on:click={logout}>
				<span class="pr-1"><Logout height="22" width="18" /></span>
				Logout
			</a>
		</span>
		<img
			alt="avatar"
			class="inline w-8 h-8 rounded-full"
			src={$identity.picture || defaultPicture}
			width="384"
			height="512"
		/>
	{:else}
		<span class="text-sky-200">
			<a class="flex pt-1 pr-1" href={'#'} on:click={login}>
				<span class="pr-1"><Login height="22" width="18" /></span>
				Sign in
			</a>
		</span>
	{/if}
</span>
