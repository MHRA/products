import React from 'react';
import { Normalize } from 'styled-normalize';
import Header from './components/header';

const app: React.FC = () => {
  return (
    <>
      <Normalize />
      <Header />
    </>
  );
};

export default app;
