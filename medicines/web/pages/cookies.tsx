import React from 'react';
import styled from 'styled-components';
import Page from '../components/page';
// @ts-ignore
import cookies from '../copy/cookies.md';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

const StyledMain = styled.main`
  padding: ${baseSpace};
  font-size: 19px;
  line-height: 28px;

  a {
    color: ${mhra70};
  }
`;

const App: React.FC = () => {
  return (
    <Page title="Cookie Policy">
      <StyledMain dangerouslySetInnerHTML={{ __html: cookies }} />
    </Page>
  );
};

export default App;
