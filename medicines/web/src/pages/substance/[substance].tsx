import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import { NextPage, NextPageContext } from 'next';
import substanceLoader from '../../services/substance-loader';
import { ISubstance } from '../../model/substance';
import DrugIndex from '../../components/drug-index';

interface IAppProps {
  results: ISubstance[];
  substanceName: string;
}

const App: NextPage<IAppProps> = props => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const router = useRouter();

  useEffect(() => {
    if (!props.substanceName) {
      router.push('/');
    }
  }, [props.substanceName]);

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
          title={`${props.substanceName || '...'}`}
          items={props.results}
        />
      </SearchWrapper>
    </Page>
  );
};

App.getInitialProps = async (context: NextPageContext): Promise<IAppProps> => {
  const {
    query: { substance },
  } = context;
  let results = [];
  let substanceName = '';

  if (substance) {
    substanceName = decodeURIComponent(substance.toString());
    const firstLetter = substanceName.charAt(0);
    const substanceIndex = await substanceLoader.load(firstLetter);
    const substanceMatch = substanceIndex.find(s => s.name === substanceName);
    if (substanceMatch) {
      results = substanceMatch.products;
    }
  }

  return {
    results,
    substanceName,
  };
};

export default App;
