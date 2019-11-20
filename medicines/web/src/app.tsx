import React from 'react';
import { Normalize } from 'styled-normalize';
import Header from './components/header';
import Mip from './components/mip';

const App: React.FC = () => {
  return (
    <>
      <Normalize />
      <Header />
      <Mip />
    </>
  );
};

export default App;
