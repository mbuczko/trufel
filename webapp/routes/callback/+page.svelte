<script>
import auth from '$lib/authService';
import { onMount } from 'svelte';

onMount(() => {
    let authClient = auth.createClient().then((a) => { console.log('dupa'); })
    /* authClient.loadUserProfile().then((p) => {
*     console.log(p)
* }) */
});

</script>
