import React, { useEffect } from 'react';

import MipText from '../../components/mip-text';
import Page from '../../components/page';
import SearchWrapper from '../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../hooks';
import Events from '../../services/events';

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  useEffect(() => {
    Events.viewPage('bmgf homepage');
  }, []);

  return (
    <Page
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy"
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
