import React, { useEffect } from 'react';
import styled from 'styled-components';
import Page from '../components/page';
// @ts-ignore
import cookies from '../copy/cookies.md';
import Events from '../services/events';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

const StyledMain = styled.main`
  padding: ${baseSpace};
  font-size: 19px;
  line-height: 28px;

  a {
    color: ${mhra70};
  }

  table {
    border-collapse: collapse;
    width: 100%;
  }

  thead {
    border-bottom: 1px solid;
  }

  th {
    text-align: start;
  }

  table,
  th,
  td {
    padding: 1rem;
  }
`;

const App: React.FC = () => {
  useEffect(() => Events.page('cookies'));

  return (
    <Page title="Cookie Policy">
      <StyledMain dangerouslySetInnerHTML={{ __html: cookies }} />
    </Page>
  );
};

export default App;
