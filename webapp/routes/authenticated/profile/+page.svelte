<script>
import { onMount, getContext } from 'svelte';
import Pusher from 'pusher-js';

const { getAuthClient } = getContext('auth');

let firstName;
let lastName;

onMount(() => {
	initPusher();
	getAuthClient()
		.loadUserProfile()
		.then((profile) => {
			firstName = profile.firstName;
			lastName = profile.lastName;
		});
});

function initPusher() {
	const client = new Pusher('8b48d3be-85f9-4919-bce6-dbf88472d30f', {
		wsHost: 'pusher.rodzinks.pl',
		wsPort: 6001,
		cluster: '',
		forceTLS: true,
		enabledTransports: ['ws', 'wss'],
		channelAuthorization: {
			endpoint: 'http://localhost:3030/pusher/auth',
			transport: 'ajax',
			params: {},
			headers: {
				Authorization: 'Bearer ' + getAuthClient().token,
				Accept: 'application/json'
			}
		}
	});
	/* const channel = client.subscribe('private-chat-room').bind('client-message', (message) => {
	   console.log(`${message.sender} says: ${message.content}`);
	   }); */
	const channel = client.subscribe('private-chat-room').bind_global((eventName, data) => {
        console.log(eventName, data);
    });
	client.connection.bind('connected', function (e) {
        console.info('Connected to channel')
	});
	client.connection.bind('error', function (e) {
		if (e.error && e.error.data && e.error.data.code === 4004) {
		    console.error('Over limit!', e);
		}
	});
}
</script>

<div>
	Logged as: {firstName}
	{lastName}
</div>
