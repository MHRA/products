import React from 'react';
import styled from 'styled-components';
import CookieForm from '../components/cookie-form';
import Page from '../components/page';
// @ts-ignore
import cookiesTable from '../copy/cookies-table.md';
// @ts-ignore
import cookies from '../copy/cookies.md';
import { useLocalStorage } from '../hooks';
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
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  return (
    <Page
      title="Cookie Policy"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <StyledMain dangerouslySetInnerHTML={{ __html: cookies }} />
      <CookieForm
        storageAllowed={storageAllowed}
        setStorageAllowed={setStorageAllowed}
      />
      <StyledMain dangerouslySetInnerHTML={{ __html: cookiesTable }} />
    </Page>
  );
};

export default App;
