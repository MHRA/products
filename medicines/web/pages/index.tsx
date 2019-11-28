import React from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';
import Footer from '../components/footer';
import Header from '../components/header';
import Mip from '../components/mip';

const StyledApp = styled.div`
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

const App: React.FC = () => {
  return (
    <StyledApp>
      <Normalize />
      <Header />
      <Mip />
      <Footer />
    </StyledApp>
  );
};

export default App;
