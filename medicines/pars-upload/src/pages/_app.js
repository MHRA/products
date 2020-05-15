import 'govuk-frontend/govuk/all.scss';
import Head from 'next/head';
import { useState, useEffect } from 'react';
import { SignInRequest } from '../auth/signInRequest';
import { Header } from '../header';
import { Footer } from '../footer';
import { signIn, getAccount } from '../auth/authPopup';

function App({ Component, pageProps }) {
  const [auth, setAuth] = useState(null);

  const triggerSignIn = () => {
    signIn().then(setAuth);
  };

  useEffect(() => {
    setAuth(getAccount());
  }, []);

  const signOut = () => {
    if (auth) {
      auth.signOut();
    }
    setAuth(null);
  };

  return (
    <>
      <Head>
        <title>Public Assessment Reports (PARs) upload</title>
      </Head>

      <Header
        account={auth ? auth.account : null}
        signOut={signOut}
        signIn={triggerSignIn}
      />

      {auth ? (
        <Component {...pageProps} />
      ) : (
        <SignInRequest
          signIn={triggerSignIn}
          onSignIn={(auth) => setAuth(auth)}
        />
      )}

      <Footer />
    </>
  );
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

export default App;
