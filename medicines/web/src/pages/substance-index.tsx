import React, { useEffect } from 'react';
import { useRouter } from 'next/router';

import DrugIndex from '../components/drug-index/index';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import { useLocalStorage } from '../hooks';
import { NextPage } from 'next';
import substanceLoader from '../services/substance-loader';
import { ISubstance } from '../model/substance';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<ISubstance[]>([]);
  const [substanceIndex, setSubstanceIndex] = React.useState();

  const router = useRouter();
  const {
    query: { index },
  } = router;

  useEffect(() => {
    if (!index) {
      return;
    }
    (async () => {
      setResults(await substanceLoader.load(index.toString()));
      setSubstanceIndex(index.toString());
    })();
  }, [index]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <DrugIndex title={`${substanceIndex || '...'}`} items={results} />
      </SearchWrapper>
    </Page>
  );
};

export default App;
