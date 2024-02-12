<script>
import { identity } from '$lib/store.js';
import Spinner from '$lib/Spinner.svelte';
import Login from 'svelte-material-icons/Login.svelte';
import Logout from 'svelte-material-icons/Logout.svelte';
import { getContext } from 'svelte';

const defaultPicture = '/avatar.jpg';
const { login, logout } = getContext('auth');
</script>

<div class="session-button flex items-center">
	{#if $identity.authenticating}
		
	{:else if $identity.id}
		<div class="profile-name text-sky-200">
			<span>{$identity.firstName}</span>
		</div>
		<div class="session-link pr-3 text-slate-400">
			<a href={'#'} on:click={logout}>
                <Logout /><span>Logout</span>
			</a>
		</div>
		<img
			alt="avatar"
			class="inline w-8 h-8 rounded-full"
			src={($identity.attributes && $identity.attributes.picture) || defaultPicture}
			width="384"
			height="512"
		/>
	{:else}
		<div class="session-link text-sky-200">
			<a href={'#'} on:click={login}>
				<Login /><span>Sign in</span>
			</a>
		</div>
	{/if}
</div>

<style>
 .session-button .profile-name::after {
     padding: 0 0.5rem;
     color: gray;
     content: "::";
 }
 .session-button .session-link a {
     display: flex;
     align-items: center;
     gap: 4px;
 }
</style>
