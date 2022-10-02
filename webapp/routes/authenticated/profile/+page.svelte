<script>
import { onMount, getContext } from 'svelte';

const { getAuthClient } = getContext('authClient');

let firstName;
let lastName;

onMount(() => {
	getAuthClient()
		.loadUserProfile()
		.then((profile) => {
			firstName = profile.firstName;
			lastName = profile.lastName;
			console.log('profile', profile);
		});
});
</script>

<div>
	Logged as: {firstName}
	{lastName}
</div>
