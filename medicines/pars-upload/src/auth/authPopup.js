import { UserAgentApplication } from 'msal'
import { msalConfig, loginRequest } from './authConfig'

export async function getAccount() {
  const msalInstance = new UserAgentApplication(msalConfig)
  const account = msalInstance.getAccount()

  if (account) {
    const token = (await getToken(msalInstance)).accessToken
    console.log({ token })

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
