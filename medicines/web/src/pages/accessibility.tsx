import React, { useEffect } from 'react';
import styled from 'styled-components';
import Page from '../components/page';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

// @ts-ignore
import accessibility from '../copy/accessibility.md';
import { useLocalStorage } from '../hooks';
import Events from '../services/events';

const StyledMain = styled.div`
  padding: ${baseSpace};
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

  useEffect(() => Events.viewPage('accessibility'));

  return (
    <Page
      title="Accessibility Statement"
      metaTitle="Products | Accessibility Statement"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <StyledMain dangerouslySetInnerHTML={{ __html: accessibility }} />
    </Page>
  );
};

export default App;
