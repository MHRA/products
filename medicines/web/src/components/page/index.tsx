import Head from 'next/head';
import React, { useEffect } from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';

import Events from '../../services/events';
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
  metaTitle: string;
  storageAllowed: boolean;
  setStorageAllowed: any;
}

const IS_PRODUCTION = process.env.ENV === 'production';
const SHOW_BMGF = process.env.SHOW_BMGF === 'true';

const App: React.FC<IPageProps> = (props) => {
  useEffect(() => {
    if (props.storageAllowed) {
      Events.initializeTrackingScripts();
    }
  }, [props.storageAllowed]);

  return (
    <>
      <Head>
        <title>MHRA {props.metaTitle}</title>
        {IS_PRODUCTION ? (
          <></>
        ) : (
          <meta name="robots" content="noindex, no follow" />
        )}
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

export const BmgfPage: React.FC<IPageProps> = (props) => (
  <>
    <Head>
      {SHOW_BMGF ? <></> : <meta name="robots" content="noindex, no follow" />}
    </Head>
    <App {...props} />
  </>
);

export default App;
