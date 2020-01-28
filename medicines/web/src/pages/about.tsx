import React from 'react';
import styled from 'styled-components';
import Page from '../components/page';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

// @ts-ignore
import about from '../copy/about.md';
import { useLocalStorage } from '../hooks';

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
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <StyledMain dangerouslySetInnerHTML={{ __html: about }} />
    </Page>
  );
};

export default App;
