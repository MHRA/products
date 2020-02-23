import { useRouter } from 'next/router';
import React from 'react';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import Document, {
  DocumentContext,
  Head,
  Html,
  Main,
  NextScript,
} from 'next/document';
import { NextPage, NextPageContext } from 'next';

interface IAppProps {
  searchResults: (docContext: DocumentContext): 
}

const App: NextPage<IAppProps> = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  const router = useRouter();
  let initialSearchValue = router.query.q;

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue={initialSearchValue}>
        <div></div>
      </SearchWrapper>
    </Page>
  );
};

App.getInitialProps = async function() {
  const res = await fetch('https://api.tvmaze.com/search/shows?q=batman');
  const data = await res.json();

  console.log(`Show data fetched. Count: ${data.length}`);

  return {
    shows: data.map(entry => entry.show)
  };
};

export default App;
