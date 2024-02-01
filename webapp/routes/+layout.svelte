<script>
import SessionButton from '$lib/SessionButton.svelte';
import SecurityContext from '$lib/SecurityContext.svelte';
</script>

<SecurityContext>
	<nav id="top-nav">
		<div class="flex items-center header bg-slate-600 shadow-lg p-2">
			<div class="basis-60 mx-6 w-60">
				<span
					class="uppercase text-gray-100 text-xl font-semibold font-mono ml-9"
					style="text-shadow: 1px 1px #333"
				>
					Trufel :>
				</span>
			</div>
            <div class="flex-1">
            </div>
			<div class="text-slate-500">
				<SessionButton />
			</div>
		</div>
	</nav>
	<slot />
</SecurityContext>
