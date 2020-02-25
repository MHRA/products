import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import MipText from '../components/mip-text';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import { useLocalStorage } from '../hooks';

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  const router = useRouter();
  const {
    query: { search, page, substance, disclaimer, doc },
  } = router;

  useEffect(() => {
    if (search) {
      const pageNum = page || 1;
      router.push({
        pathname: '/search',
        query: { q: search.toString(), page: pageNum, doc, disclaimer },
      });
    } else if (substance) {
      if (substance.length === 1) {
        router.push({
          pathname: '/product-index',
          query: { index: substance },
        });
      } else {
        router.push({ pathname: '/substance', query: { substance } });
      }
    }
  }, [search, page, substance, disclaimer, doc]);

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
