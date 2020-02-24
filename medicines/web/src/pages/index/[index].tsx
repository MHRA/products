import React, { useEffect } from 'react';
import { NextPage, NextPageContext } from 'next';
import { useRouter } from 'next/router';

import DrugIndex from '../../components/drug-index/index';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import substanceLoader from '../../services/substance-loader';
import { ISubstance } from '../../model/substance';

interface IAppProps {
  results: ISubstance[];
  substanceIndex: string;
}

const App: NextPage<IAppProps> = props => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const router = useRouter();

  useEffect(() => {
    if (!props.substanceIndex) {
      router.push('/');
    }
  }, [props.substanceIndex]);

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
          title={`${props.substanceIndex || '...'}`}
          items={props.results}
        />
      </SearchWrapper>
    </Page>
  );
};

App.getInitialProps = async (context: NextPageContext): Promise<IAppProps> => {
  const {
    query: { index },
  } = context;
  let results = [];
  let substanceIndex = '';

  if (index) {
    substanceIndex = index.toString();
    results = await substanceLoader.load(substanceIndex);
  }

  return {
    results,
    substanceIndex,
  };
};

export default App;
