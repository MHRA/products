import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest } from './authConfig'

export async function getAccount() {
  msalConfig.auth.redirectUri = getCurrentHost(window.location.href)
  const msalInstance = new UserAgentApplication(msalConfig)
  const account = msalInstance.getAccount()

  if (account) {
    const tokenSessionStorageKey = `msal.${msalConfig.auth.clientId}.idtoken`
    const token = window.sessionStorage[tokenSessionStorageKey]
    const username = account.userName

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

const isIE = () => {
  const ua = window.navigator.userAgent
  const msie = ua.indexOf('MSIE ')
  const msie11 = ua.indexOf('Trident/')
  return msie > 0 || msie11 > 0
}

export function getCurrentHost(currentFullUrl) {
  var uriComponents = currentFullUrl.split('/')
  return uriComponents[0] + '//' + uriComponents[2]
}

export function signIn() {
  msalConfig.auth.redirectUri = getCurrentHost(window.location.href)

  if (isIE()) {
    new UserAgentApplication(msalConfig).loginRedirect(loginRequest)
  } else {
    return new UserAgentApplication(msalConfig)
      .loginPopup(loginRequest)
      .then(getAccount)
  }
}
