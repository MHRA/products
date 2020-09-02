import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest, tokenRequest } from './authConfig'

export async function getAccount() {
  msalConfig.auth.redirectUri = getCurrentHost(window.location.href)
  const msalInstance = new UserAgentApplication(msalConfig)
  const account = msalInstance.getAccount()

  if (msalInstance && account) {
    let auth = await msalInstance
      .acquireTokenSilent(tokenRequest)
      .then((response) => {
        return {
          account,
          token: response.accessToken,
          username: account.userName,
          signOut: () => {
            msalInstance.logout()
          },
        }
      })
    return auth
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
