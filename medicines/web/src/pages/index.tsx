import React from 'react';
import Mip from '../components/mip';
import Page from '../components/page';
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
      <Mip />
    </Page>
  );
};

export default App;
