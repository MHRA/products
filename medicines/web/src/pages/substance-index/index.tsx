import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import DrugIndex, { IndexType } from '../../components/drug-index/index';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { SubstanceListStructuredData } from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { ISubstance } from '../../model/substance';
import Events from '../../services/events';
import { getLoader } from '../../services/loaders/products/substances-index-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<ISubstance[]>([]);
  const [substanceIndex, setSubstanceIndex] = React.useState('');
  const [isLoading, setIsLoading] = React.useState(true);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);
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

    setErrorFetchingResults(false);
    setIsLoading(true);
    setResults([]);

    const loader = getLoader(useGraphQl);
    loader
      .load(index)
      .then((results) => {
        setResults(results);
        setIsLoading(false);
      })
      .catch((e) => {
        setErrorFetchingResults(true);
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
      metaTitle="Products | Substance index"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <DrugIndex
          title={`${substanceIndex || '...'}`}
          items={results}
          indexType={IndexType.SubstancesIndex}
          errorFetchingResults={errorFetchingResults}
          isLoading={isLoading}
        />
        <SubstanceListStructuredData
          substanceNames={results.map((substance) => substance.name)}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
