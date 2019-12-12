import React from 'react';
import styled from 'styled-components';
import Page from '../components/page';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

// @ts-ignore
import about from '../copy/about.md';

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
    <Page title="Products">
      <StyledMain dangerouslySetInnerHTML={{ __html: about }} />
    </Page>
  );
};

export default App;
