import React from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';

import { nibscAccessibleGreen, anchorColour } from '../../styles/colors';
import { desktopMaxWidth } from '../../styles/dimensions';

import Footer from '../footer';
import Head from '../head';
import Header from '../header';

const WithStyles = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
  font-size: 16px;

  * {
    box-sizing: border-box;
  }

  picture {
    display: block;
  }

  img,
  svg {
    height: auto;
    max-width: 100%;
  }

  a {
    color: ${anchorColour};
    text-decoration: underline;
    &:hover {
      color: ${nibscAccessibleGreen};
    }
  }
`;

const PageWrapper = styled.main`
  flex-grow: 1;
  margin: 0 auto 4rem;
  max-width: ${desktopMaxWidth};
  width: 100%;
`;

interface IPageProps {
  children: React.ReactNode;
  title: string;
}

const App: React.FC<IPageProps> = (props) => {
  return (
    <>
      <Head title={props.title} />
      <WithStyles>
        <Normalize />
        <Header />
        <PageWrapper>{props.children}</PageWrapper>
        <Footer />
      </WithStyles>
    </>
  );
};

export default App;
