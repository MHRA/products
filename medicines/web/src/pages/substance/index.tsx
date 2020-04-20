import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import Page from '../../components/page';
import ProductList from '../../components/product-list/index';
import SearchWrapper from '../../components/search-wrapper';
import {
  DrugListStructuredData,
  SubstanceStructuredData,
} from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { IProduct } from '../../model/substance';
import Events from '../../services/events';
import graphQl from '../../services/graphql-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [products, setProducts] = React.useState<IProduct[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');

  const router = useRouter();
  const {
    query: { substance: queryQS },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    (async () => {
      const substanceStr = queryQS.toString();
      graphQl.products.load(substanceStr).then(responseData => {
        setProducts(responseData);
        setSubstanceName(substanceStr);
        Events.viewProductsForSubstance(substanceStr);
      });
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
        <ProductList title={substanceName} products={products} />
        <SubstanceStructuredData substanceName={substanceName} />
        <DrugListStructuredData
          drugNames={products.map(product => product.name)}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
