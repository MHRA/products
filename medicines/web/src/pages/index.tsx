import React from 'react';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import MipText from '../components/mip-text';
import { useLocalStorage } from '../hooks';

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
      <SearchWrapper initialSearchValue="">
        <MipText />
      </SearchWrapper>
    </Page>
  );
};

export default App;
