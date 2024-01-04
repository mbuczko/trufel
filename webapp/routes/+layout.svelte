<script>
import LogButton from '$lib/LogButton.svelte';
import SecurityContext from '$lib/SecurityContext.svelte';
</script>

<SecurityContext>
	<nav id="top-nav">
		<div class="flex header bg-slate-600 shadow-lg p-2">
			<div class="grow mx-6 w-60 align-baseline mt-1">
				<span
					class="uppercase text-gray-100 text-xl font-semibold font-mono ml-9"
					style="text-shadow: 1px 1px #333"
				>
					Trufel :>
				</span>
			</div>
			<div class="w-70 text-right text-slate-500">
				<LogButton />
			</div>
		</div>
	</nav>
	<slot />
</SecurityContext>
