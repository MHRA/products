import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import DrugIndex, {
  IndexType,
} from '../../../components/bmgf/drug-index/index';
import { BmgfPage } from '../../../components/page';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { SubstanceListStructuredData } from '../../../components/structured-data';
import { useLocalStorage } from '../../../hooks';
import { IFacet } from '../../../model/facet';
import Events from '../../../services/events';
import { getLoader } from '../../../services/loaders/medicine-levels-in-pregnancy/substances-index-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<IFacet[]>([]);
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

    getLoader(useGraphQl)
      .load(index)
      .then((results) => {
        setResults(results);
        setIsLoading(false);
      })
      .catch((e) => {
        setErrorFetchingResults(true);
      });

    Events.viewPbpkSubstancesStartingWith(index);
  }, [queryQS]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  return (
    <BmgfPage
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy | Substance index"
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
    </BmgfPage>
  );
};

export default App;
