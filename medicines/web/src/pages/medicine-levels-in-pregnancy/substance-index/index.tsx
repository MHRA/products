import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import DrugIndex, {
  IndexType,
} from '../../../components/bmgf/drug-index/index';
import Page from '../../../components/page';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { SubstanceListStructuredData } from '../../../components/structured-data';
import { useLocalStorage } from '../../../hooks';
import { ISubstanceIndex } from '../../../model/substance';
import Events from '../../../services/events';
import {
  substancesIndexLoader,
  graphqlSubstancesIndexLoader,
} from '../../../services/loaders/medicine-levels-in-pregnancy/substances-index-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<ISubstanceIndex[]>([]);
  const [substanceIndex, setSubstanceIndex] = React.useState('');
  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { letter: queryQS },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    const index = queryQS.toString();
    setSubstanceIndex(index);

    const loader = useGraphQl
      ? graphqlSubstancesIndexLoader
      : substancesIndexLoader;

    loader.load(index).then((results) => {
      results = results || [];
      setResults(results);
    });

    Events.viewSubstancesStartingWith(index);
  }, [queryQS]);

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
        <DrugIndex
          title={`${substanceIndex || '...'}`}
          items={results}
          indexType={IndexType.SubstancesIndex}
        />
        <SubstanceListStructuredData
          substanceNames={results.map((substance) => substance.name)}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
