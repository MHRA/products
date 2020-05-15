import 'govuk-frontend/govuk/all.scss';
import Head from 'next/head';
import { SignInRequest } from '../auth/signInRequest';
import { useState } from 'react';

function App({ Component, pageProps }) {
  const [account, setAccount] = useState(0);

  console.log('xxxxxxx');
  console.log(account);
  return (
    <>
      <Head>
        <title>Public Assessment Reports (PARs) upload</title>
      </Head>
      {account ? (
        <Component {...pageProps} />
      ) : (
        <SignInRequest onSignIn={(account) => setAccount(account)} />
      )}
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
