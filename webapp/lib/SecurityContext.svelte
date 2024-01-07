<script>
// @ts-nocheck

import { onMount, setContext } from 'svelte';
import { identity, keycloak } from '$lib/store.js';
import Keycloak from 'keycloak-js';

onMount(() => {
    const kc = new Keycloak({
        url: 'https://auth.rodzinks.pl/',
        realm: 'rodzinks',
        clientId: 'trufel-webapp'
    });

    keycloak.set(kc);
    identity.update((v) => {
        v.id = null;
        v.authenticating = true;
        return v;
    });

    kc
        .init({
            onLoad: 'check-sso',
            silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso'
        })
        .then((authenticated) => {
            if (authenticated) {
                return kc.loadUserProfile();
            }
        })
        .then((response) => {
            if (!response || !response.id) {
                response = { authenticating: false };
            }
            identity.set(response);
            return response;
        });
});

setContext('auth', {
    getAuthClient: () => $keycloak,
    getIdentity: () => $identity,
    login: () => $keycloak.login({ redirectUri: 'http://localhost:5173/' }),
    logout: () => $keycloak.logout({ redirectUri: 'http://localhost:5173/' })
});
</script>

<slot />
