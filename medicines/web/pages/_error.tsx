import { default as NextError } from 'next/error';
import Router from 'next/router';
import React from 'react';
import ReactGA from 'react-ga-gtm';

// @ts-ignore
function Error({ statusCode: statusCode }) {
  ReactGA.event({
    category: 'error',
    action: `User navigated to '${Router.asPath}' and got a ${statusCode} error.`,
  });

  return <NextError statusCode={statusCode} />;
}

export default Error;
