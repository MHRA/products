import '../global-styles.scss'
import { useState, useEffect } from 'react'
import { SignInRequest } from '../auth/signInRequest'
import { Header } from '../header'
import { Footer } from '../footer'
import { signIn, getAccount } from '../auth/authPopup'

function App({ Component, pageProps }) {
  const [auth, setAuth] = useState(null)
  const [authError, setAuthError] = useState(null)

  const triggerSignIn = () => {
    signIn()
      .then(setAuth)
      .catch((e) => handleAuthError(e.toString()))
  }

  function handleAuthError(error) {
    if (error.startsWith('ClientAuthError:')) {
      error = error.slice(16)

      if (error.includes('Login_In_Progress')) {
        error =
          'Login popup is already open in another window, perhaps it is behind this window. Complete login on that window and then refresh this page.'
      }

      setAuthError(error)
    } else {
      console.error('Auth error:', error)
      setAuthError('Unknown error')
    }
  }

  useEffect(() => {
    getAccount().then(setAuth)
  }, [])

  const signOut = () => {
    if (auth) {
      auth.signOut()
    }

    setAuth(null)
  }

  return (
    <>
      <Header
        account={auth ? auth.account : null}
        signOut={signOut}
        signIn={triggerSignIn}
      />
      {process.env.NEXT_PUBLIC_DISABLE_AUTH === 'true' || auth ? (
        <Component {...pageProps} auth={auth} />
      ) : (
        <SignInRequest
          signIn={triggerSignIn}
          onSignIn={(auth) => setAuth(auth)}
          error={authError}
        />
      )}
      <Footer />
    </>
  )
}

// Only uncomment this method if you have blocking data requirements for
// every single page in your application. This disables the ability to
// perform automatic static optimization, causing every page in your app to
// be server-side rendered.
//
// MyApp.getInitialProps = async (appContext) => {
//   // calls page's `getInitialProps` and fills `appProps.pageProps`
//   const appProps = await App.getInitialProps(appContext);
//
//   return { ...appProps }
// }

export default App
