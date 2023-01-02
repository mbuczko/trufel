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
            console.log(profile);
		});
});

function initPusher() {
	const client = new Pusher('8b48d3be-85f9-4919-bce6-dbf88472d30f', {
		wsHost: 'localhost',
		wsPort: 6001,
		cluster: '',
		forceTLS: false,
		enabledTransports: ['ws', 'wss'],
		channelAuthorization: {
			endpoint: 'http://localhost:3030/pusher/auth',
            transport: 'ajax',
			params: {},
			headers: {
                'Authorization': 'Bearer ' + getAuthClient().token,
                'Accept': 'application/json'
            }
		}
	});
	const channel = client.subscribe('private-chat-room').bind('message', (message) => {
		console.log(`${message.sender} says: ${message.content}`);
	});
	client.connection.bind('error', function (err) {
		if (err.error.data.code === 4004) {
			log('Over limit!');
		}
	});
    channel.trigger('client-my-event', {message: 'Hello, world!'})
}
</script>

<div>
	Logged as: {firstName}
	{lastName}
</div>
