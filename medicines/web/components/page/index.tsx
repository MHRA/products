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

const Row = styled.section`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 0 auto;
  flex-grow: 1;
  max-width: ${desktopMaxWidth};
  > * {
    flex-basis: 100%;
    flex-shrink: 1;
    flex-grow: 1;
  }

  @media ${mobileBreakpoint} {
    display: block;
  }
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
        <Row>{props.children}</Row>
        <Footer />
      </WithStyles>
    </>
  );
};

export default App;
