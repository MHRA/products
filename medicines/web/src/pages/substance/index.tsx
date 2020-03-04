import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import { JsonLd } from 'react-schemaorg';
import { ItemList, Substance } from 'schema-dts';
import DrugIndex from '../../components/drug-index/index';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import { IProduct } from '../../model/substance';
import Events from '../../services/events';
import substanceLoader from '../../services/substance-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<IProduct[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');

  const router = useRouter();
  const {
    query: { query: queryQS },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    (async () => {
      const substanceStr = queryQS.toString();
      const firstLetter = substanceStr.charAt(0);
      const substanceIndex = await substanceLoader.load(firstLetter);
      const substanceMatch = substanceIndex.find(s => s.name === substanceStr);
      if (substanceMatch) {
        setResults(substanceMatch.products);
      }
      setSubstanceName(substanceStr);
      Events.viewProductsForSubstance(substanceStr);
    })();
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
        <DrugIndex title={`${substanceName || '...'}`} items={results} />
        <JsonLd<Substance>
          item={{
            '@context': 'https://schema.org',
            '@type': 'Substance',
            name: substanceName,
          }}
        />
        <JsonLd<ItemList>
          item={{
            '@context': 'https://schema.org',
            '@type': 'ItemList',
            itemListElement: results.map((product, index) => {
              return {
                '@type': 'ListItem',
                position: index,
                item: {
                  '@type': 'Drug',
                  name: product.name,
                  url:
                    'https://products.mhra.gov.uk/product?query=' +
                    encodeURIComponent(product.name),
                },
              };
            }),
          }}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
