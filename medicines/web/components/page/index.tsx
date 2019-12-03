import Head from 'next/head';
import React from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';
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
`;

interface IPageProps {
  children: React.ReactNode;
  title: string;
}

const App: React.FC<IPageProps> = props => {
  return (
    <>
      <Head>
        <title>{props.title}</title>
      </Head>
      <WithStyles>
        <Normalize />
        <Header />
        {props.children}
        <Footer />
      </WithStyles>
    </>
  );
};

export default App;
