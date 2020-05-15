import * as Msal from 'msal';
import { msalConfig, loginRequest } from './authConfig';

// Create the main myMSALObj instance
// configuration parameters are located at authConfig.js

export function signIn() {
  const myMSALObj = new Msal.UserAgentApplication(msalConfig);

  return myMSALObj.loginPopup(loginRequest).then((loginResponse) => {
    console.log('id_token acquired at: ' + new Date().toString());
    console.log(loginResponse);

    return myMSALObj.getAccount();
    // if (myMSALObj.getAccount()) {
    //   return myMSALObj.getAccount();
    // } else {
    //   throw new Error('Failed to authenticate.');
    // }
  });
}

function signOut() {
  myMSALObj.logout();
}

function getTokenPopup(request) {
  return myMSALObj.acquireTokenSilent(request).catch((error) => {
    console.log(error);
    console.log('silent token acquisition fails. acquiring token using popup');

    // fallback to interaction when silent call fails
    return myMSALObj
      .acquireTokenPopup(request)
      .then((tokenResponse) => {
        return tokenResponse;
      })
      .catch((error) => {
        console.log(error);
      });
  });
}

function seeProfile() {
  if (myMSALObj.getAccount()) {
    getTokenPopup(loginRequest)
      .then((response) => {
        callMSGraph(
          graphConfig.graphMeEndpoint,
          response.accessToken,
          updateUI,
        );
        profileButton.classList.add('d-none');
        mailButton.classList.remove('d-none');
      })
      .catch((error) => {
        console.log(error);
      });
  }
}

function readMail() {
  if (myMSALObj.getAccount()) {
    getTokenPopup(tokenRequest)
      .then((response) => {
        callMSGraph(
          graphConfig.graphMailEndpoint,
          response.accessToken,
          updateUI,
        );
      })
      .catch((error) => {
        console.log(error);
      });
  }
}
