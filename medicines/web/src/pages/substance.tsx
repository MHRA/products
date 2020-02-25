import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import { useLocalStorage } from '../hooks';
import { NextPage } from 'next';
import substanceLoader from '../services/substance-loader';
import { ISubstance, IProduct } from '../model/substance';
import DrugIndex from '../components/drug-index/index';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<IProduct[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');

  const router = useRouter();
  const {
    query: { substance },
  } = router;

  useEffect(() => {
    if (!substance) {
      return;
    }
    (async () => {
      const substanceStr = substance.toString();
      const firstLetter = substanceStr.charAt(0);
      const substanceIndex = await substanceLoader.load(firstLetter);
      const substanceMatch = substanceIndex.find(s => s.name === substanceStr);
      if (substanceMatch) {
        setResults(substanceMatch.products);
      }
      setSubstanceName(substanceStr);
    })();
  }, [substance]);

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
        <DrugIndex title={`${substanceName || '...'}`} items={results} />
      </SearchWrapper>
    </Page>
  );
};

export default App;
