<script>
import { onMount, getContext } from 'svelte';

const { getAuthClient } = getContext('authClient');

onMount(() => {
	console.log(
		getAuthClient()
			.loadUserProfile()
			.then((profile) => { 
				console.log(profile);
			})
	);
});
</script>
