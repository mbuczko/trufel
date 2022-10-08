<script>
import { identity } from '$lib/store.js';

const picture =
	($identity && $identity.picture) ||
	'https://tailwindcss.com/_next/static/media/sarah-dayan.a620c98f.jpg';

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
			{#if $identity}
				<span class="p-1 text-sky-200">
					{$identity.name}
				</span>
				|
				<a class="p-1 pr-3 text-slate-400" href={'#'} on:click={logout}>Logout</a>
				<img
					alt="avatar"
					class="inline w-8 h-8 rounded-full"
					src={picture}
					width="384"
					height="512" />
			{:else}
				<a href={'#'} on:click={login}>Sign in</a>
			{/if}
		</div>
	</div>
</nav>

<slot/>
