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
import { IProduct } from '../../model/product';
import Events from '../../services/events';
import { getLoader } from '../../services/loaders/products/products-index-loader';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [products, setProducts] = React.useState<IProduct[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');
  const [isLoading, setIsLoading] = React.useState(true);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);
  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';
  const router = useRouter();
  const {
    query: { substance: queryQS },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    const substanceName = queryQS.toString();
    setSubstanceName(substanceName);

    setProducts([]);
    setIsLoading(true);
    setErrorFetchingResults(false);

    const loader = getLoader(useGraphQl);

    loader
      .load(substanceName)
      .then((products) => {
        setProducts(products);
        setIsLoading(false);
      })
      .catch((e) => setErrorFetchingResults(true));

    Events.viewProductsForSubstance(substanceName);
  }, [queryQS]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  return (
    <Page
      title="Products"
      metaTitle="Products | Substance"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <ProductList
          title={substanceName}
          products={products}
          errorFetchingResults={errorFetchingResults}
          isLoading={isLoading}
        />
        <SubstanceStructuredData substanceName={substanceName} />
        {products && products.length ? (
          <DrugListStructuredData
            drugNames={products.map((product) => product.name)}
          />
        ) : (
          <></>
        )}
      </SearchWrapper>
    </Page>
  );
};

export default App;
