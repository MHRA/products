import React, { useEffect } from 'react';
import styled from 'styled-components';
import Page from '../components/page';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

// @ts-ignore
import about from '../copy/about.md';
import Events from '../services/events';

const StyledMain = styled.main`
  padding: ${baseSpace};
  font-size: 19px;
  line-height: 28px;

  a {
    color: ${mhra70};
  }

  ul li {
    list-style: none;
  }
`;

const App: React.FC = () => {
  useEffect(() => Events.viewPage('about'));

  return (
    <Page title="Products">
      <StyledMain dangerouslySetInnerHTML={{ __html: about }} />
    </Page>
  );
};

export default App;
