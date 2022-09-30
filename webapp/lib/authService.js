import Keycloak from 'keycloak-js';

function createClient() {
  const authClient = new Keycloak();
  authClient.init({
    onLoad: 'check-sso',
    silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso'
  }).then((authenticated) => {
    console.log(authenticated ? "authenticated" : "NOT authenticated")
  }).catch((e) => console.error("Failed to initialize keycloak", e))
  console.log('creating');

  return authClient;
}

const auth = {
	createClient
};

export default auth;

