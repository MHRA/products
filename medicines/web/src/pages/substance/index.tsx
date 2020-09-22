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
import substanceLoader from '../../services/loaders/products/substances-index-loader';
import { graphqlProductsLoader } from '../../services/loaders/products/substance-loader';

const azureProductsLoader = async (substance: string) => {
  const firstLetter = substance.charAt(0);
  const substanceIndex = await substanceLoader.load(firstLetter);
  const substanceMatch = substanceIndex.find((s) => s.name === substance);
  if (substanceMatch) {
    return substanceMatch.products;
  }
  return [];
};

const graphQlProductsLoader = async (substance: string) => {
  return graphqlProductsLoader.load(substance);
};

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

    const loader: (substance: string) => Promise<IProduct[]> = useGraphQl
      ? graphQlProductsLoader
      : azureProductsLoader;

    loader(substanceName)
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
