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
      const docQs = doc ? `&doc=${doc}` : '';
      const disclaimerQs = disclaimer ? `&disclaimer=${disclaimer}` : '';
      router.push(
        `/search?q=${encodeURIComponent(
          search.toString(),
        )}&page=${pageNum}${docQs}${disclaimerQs}`,
      );
    } else if (substance) {
      if (substance.length === 1) {
        router.push(`/index/${substance}`);
      } else {
        router.push(`/substance/${encodeURIComponent(substance.toString())}`);
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
