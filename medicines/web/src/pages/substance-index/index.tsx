import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import DrugIndex from '../../components/drug-index/index';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import { ISubstance } from '../../model/substance';
import Events from '../../services/events';
import substanceLoader from '../../services/substance-loader';

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
      const indexStr = index.toString();
      setResults(await substanceLoader.load(indexStr));
      setSubstanceIndex(indexStr);
      Events.viewSubstancesStartingWith(indexStr);
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
