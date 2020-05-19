import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest } from './authConfig'

export function getAccount() {
  const myMSALObj = new UserAgentApplication(msalConfig)
  const account = myMSALObj.getAccount()

  if (account) {
    return {
      account,
      signOut: () => {
        myMSALObj.logout()
      },
    }
  }
}

export function signIn() {
  return new UserAgentApplication(msalConfig)
    .loginPopup(loginRequest)
    .then(getAccount)
}
