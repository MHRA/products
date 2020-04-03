import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import MipText from '../components/mip-text';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import { useLocalStorage } from '../hooks';
import Events from '../services/events';

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  const router = useRouter();
  const {
    query: {
      search: searchQS,
      page: pageQS,
      substance: substanceQS,
      disclaimer: disclaimerQS,
      doc: docQS,
      product: productQS,
    },
  } = router;

  useEffect(() => {
    Events.viewPage('homepage');
  }, []);

  useEffect(() => {
    // handle and redirect legacy urls
    if (searchQS) {
      const page = pageQS || 1;
      const pathname = productQS ? '/product' : '/search';
      router.push({
        pathname,
        query: {
          query: searchQS.toString(),
          page,
          doc: docQS,
          disclaimer: disclaimerQS,
        },
      });
    } else if (substanceQS) {
      if (substanceQS.length === 1) {
        router.push({
          pathname: '/substance-index',
          query: { substance: substanceQS },
        });
      } else {
        router.push({
          pathname: '/substance',
          query: { substance: substanceQS },
        });
      }
    }
  }, [searchQS, pageQS, substanceQS, disclaimerQS, docQS, productQS]);

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <MipText />
      </SearchWrapper>
    </Page>
  );
};

export default App;
