import { UserAgentApplication } from 'msal';
import { msalConfig, loginRequest } from './authConfig';

export function signIn() {
  const myMSALObj = new UserAgentApplication(msalConfig);

  return myMSALObj.loginPopup(loginRequest).then(() => {
    const account = myMSALObj.getAccount();

    if (account) {
      return {
        account,
        signOut: () => {
          myMSALObj.logout();
        },
      };
    } else {
      throw new Error('Failed to authenticate.');
    }
  });
}
