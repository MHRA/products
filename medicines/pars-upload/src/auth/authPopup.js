import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest } from './authConfig'

export async function getAccount() {
  const msalInstance = new UserAgentApplication(msalConfig)
  const account = msalInstance.getAccount()

  if (account) {
    // TODO:
    //
    // Temporary hack whilst we wait for implicit granting of ID tokens to be
    // enabled by MHRA admins so we get an access token in the proper way:
    // https://github.com/AzureAD/microsoft-authentication-library-for-js/tree/dev/lib/msal-core#3-get-an-access-token-to-call-an-api

    const token = window.sessionStorage['msal.idtoken']
    // const token = await getToken(msalInstance)

    return {
      account,
      token,
      signOut: () => {
        msalInstance.logout()
      },
    }
  }
}

async function getToken(msalInstance) {
  const tokenRequest = {
    scopes: ['user.read'],
  }

  try {
    return await msalInstance.acquireTokenSilent(tokenRequest)
  } catch (e) {
    if (e.name === 'InteractionRequiredAuthError') {
      return await msalInstance.acquireTokenPopup(tokenRequest)
    } else {
      throw e
    }
  }
}

export function signIn() {
  return new UserAgentApplication(msalConfig)
    .loginPopup(loginRequest)
    .then(getAccount)
}
