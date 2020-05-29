import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest } from './authConfig'

export async function getAccount() {
  msalConfig.auth.redirectUri = getCurrentHost(window.location.href)
  const msalInstance = new UserAgentApplication(msalConfig)
  const account = msalInstance.getAccount()

  if (account) {
    //const token = await getToken(msalInstance)
    const token = window.sessionStorage['msal.idtoken']
    const username = account.userName
    console.log({ token, account })

    return {
      account,
      token,
      username,
      signOut: () => {
        msalInstance.logout()
      },
    }
  }
}

export function getCurrentHost(currentFullUrl) {
  var uriComponents = currentFullUrl.split('/')
  return uriComponents[0] + '//' + uriComponents[2]
}

export function signIn() {
  msalConfig.auth.redirectUri = getCurrentHost(window.location.href)
  return new UserAgentApplication(msalConfig)
    .loginPopup(loginRequest)
    .then(getAccount)
}
