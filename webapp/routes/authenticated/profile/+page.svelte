<script>
import { onMount, getContext } from 'svelte';

const { getAuthClient } = getContext('auth');

let firstName;
let lastName;

onMount(() => {
	getAuthClient()
		.loadUserProfile()
		.then((profile) => {
			firstName = profile.firstName;
			lastName = profile.lastName;
		});
});
</script>

<div>
	Logged as: {firstName}
	{lastName}
</div>
