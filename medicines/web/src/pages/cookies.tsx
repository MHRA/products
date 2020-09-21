import React, { useEffect } from 'react';
import styled from 'styled-components';
import CookieForm from '../components/cookie-form';
import Page from '../components/page';
// @ts-ignore
import cookies from '../copy/cookies.md';
import { useLocalStorage } from '../hooks';
import Events from '../services/events';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

const StyledMain = styled.div`
  padding: ${baseSpace};
  padding-bottom: 0;
  font-size: 19px;
  line-height: 28px;

  a {
    color: ${mhra70};
  }
`;

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  useEffect(() => Events.viewPage('cookies'));

  return (
    <Page
      title="Cookie Policy"
      metaTitle="Products | Cookie Policy"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <StyledMain dangerouslySetInnerHTML={{ __html: cookies }} />
      <CookieForm
        storageAllowed={storageAllowed}
        setStorageAllowed={setStorageAllowed}
      />
    </Page>
  );
};

export default App;
