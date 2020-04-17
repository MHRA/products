import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import ProductList from '../../components/product-list/index';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import {
  DrugListStructuredData,
  SubstanceStructuredData,
} from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { IProduct } from '../../model/substance';
import Events from '../../services/events';
import substanceLoader from '../../services/substance-loader';

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
      const query = `{
        products(substanceName: "${substanceStr}") {
          name
          documentCount
        }
      }`;
      const response = await fetch("http://localhost:8000/graphql", {
        method: 'POST',
        mode: 'cors',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({"query": query, "variables": null})
      }).then((response) => {
        return response.json();
      }).then((responseData) => {
        setProducts(responseData.data.products);
        setSubstanceName(substanceStr);
        Events.viewProductsForSubstance(substanceStr);
      })
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
