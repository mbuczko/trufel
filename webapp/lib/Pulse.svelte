<script>
export let status;
$: innerClazz = status === 'ok' ? 'bg-green-400' : 'bg-red-400';
$: outerClazz = status === 'ok' ? 'bg-green-500' : 'bg-red-500';
</script>

<span class="absolute h-3 w-3 m-0.5">
	<span
		class="{innerClazz} animate-ping absolute -right-1 top-0.5 inline-flex h-5 w-5 rounded-full opacity-75"
	/>
	<span class="{outerClazz} relative inline-flex rounded-full h-3 w-3" />
</span>
