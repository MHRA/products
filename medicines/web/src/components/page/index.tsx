import Head from 'next/head';
import React, { useEffect } from 'react';
import ReactGA from 'react-ga';
import TagManager from 'react-gtm-module';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';

import { anchorColour, mhra } from '../../styles/colors';
import { desktopMaxWidth } from '../../styles/dimensions';
import CookieBanner from '../cookie-policy';
import Footer from '../footer';
import Header from '../header';

const WithStyles = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  font-family: Arial;
  font-size: 16px;

  * {
    box-sizing: border-box;
  }

  body {
  }

  picture {
    display: block;
  }

  img,
  svg {
    display: block;
    height: auto;
    max-width: 100%;
  }
  a {
    color: ${anchorColour};
    text-decoration: underline;
    &:hover {
      color: ${mhra};
    }
  }
`;

const Wrapper = styled.main`
  flex-grow: 1;
  margin: 0 auto 4rem;
  max-width: ${desktopMaxWidth};
  width: 100%;
`;

interface IPageProps {
  children: React.ReactNode;
  title: string;
  storageAllowed: boolean;
  setStorageAllowed: any;
}

const App: React.FC<IPageProps> = props => {
  useEffect(() => {
    if (props.storageAllowed) {
      TagManager.initialize({
        gtmId: process.env.GOOGLE_GTM_CONTAINER_ID as string,
        dataLayerName: 'dataLayer',
      });
      ReactGA.initialize(process.env.GOOGLE_TRACKING_ID as string, {
        debug: true,
      });
    }
  }, [props.storageAllowed]);

  return (
    <>
      <Head>
        <title>MHRA {props.title}</title>
        <meta
          httpEquiv="Content-Security-Policy-Report-Only"
          content="base-uri 'self'; default-src 'self'; script-src 'self'; style-src 'self'; object-src 'none'; form-action 'self'; font-src 'self'; connect-src 'self'; img-src 'self';"
        />
      </Head>
      <WithStyles>
        <Normalize />
        <CookieBanner
          storageAllowed={props.storageAllowed}
          setStorageAllowed={props.setStorageAllowed}
        />
        <Header title={props.title} />
        <Wrapper>{props.children}</Wrapper>
        <Footer />
      </WithStyles>
    </>
  );
};

export default App;
