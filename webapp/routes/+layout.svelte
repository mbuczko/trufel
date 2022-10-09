<script>
import { identity } from '$lib/store.js';
import Spinner from '$lib/Spinner.svelte';
import Login from 'svelte-material-icons/Login.svelte'
import Logout from 'svelte-material-icons/Logout.svelte'

const defaultPicture = '/avatar.jpg';

let nav;

function login() {
    nav.dispatchEvent(new CustomEvent('login'));
}

function logout() {
    nav.dispatchEvent(new CustomEvent('logout'));
}
</script>

<nav bind:this={nav} id="top-nav">
	<div class="flex header bg-slate-600 shadow-lg p-2">
		<div class="grow mx-6 w-60 align-baseline mt-1">
			<span
				class="uppercase text-gray-100 text-xl font-semibold font-mono ml-9"
				style="text-shadow: 1px 1px #333">
				Trufel ]:>
			</span>
		</div>
		<div class="w-70 text-right text-slate-500">
			{#if $identity && !$identity.user_id}
                <Spinner />
			{:else if $identity}
                <span class="p-1 text-sky-200">
					{$identity.name}
				</span>
				|
                <span class="p-1 pr-3 text-slate-400 inline-block align-bottom">
                    <a class="flex " href={'#'} on:click={logout}>
                        <span class="pr-1"><Logout height="22" width="18" /></span>
                        Logout
                    </a>
                </span>
				<img
					alt="avatar"
					     class="inline w-8 h-8 rounded-full"
					     src={ ($identity && $identity.picture) || defaultPicture}
					     width="384"
					     height="512" />
            {:else}
                <span class="text-sky-200">
				    <a class="flex pt-1 pr-1" href={'#'} on:click={login}><span class="pr-1"><Login height="22" width="18" /></span>Sign in</a>
                </span>
			{/if}
		</div>
	</div>
</nav>

<slot/>
