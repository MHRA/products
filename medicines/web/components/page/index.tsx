import Head from 'next/head';
import React from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';
import { anchorColour, mhra } from '../../styles/colors';
import { desktopMaxWidth, mobileBreakpoint } from '../../styles/dimensions';
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

const Wrapper = styled.section`
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
  // padding: 0 1.25rem;
`;

interface IPageProps {
  children: React.ReactNode;
  title: string;
}

const App: React.FC<IPageProps> = props => {
  return (
    <>
      <Head>
        <title>MHRA {props.title}</title>
      </Head>
      <WithStyles>
        <Normalize />
        <CookieBanner />
        <Header title={props.title} />
        <Wrapper>{props.children}</Wrapper>
        <Footer />
      </WithStyles>
    </>
  );
};

export default App;
