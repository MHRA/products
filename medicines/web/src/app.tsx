import React from 'react';
import { Normalize } from 'styled-normalize';
import Footer from './components/footer';
import Header from './components/header';
import Mip from './components/mip';

const App: React.FC = () => {
  return (
    <>
      <Normalize />
      <Header />
      <Mip />
      <Footer />
    </>
  );
};

export default App;
