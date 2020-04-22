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
import substanceLoader from '../../services/substance-loader';

const azureProductsLoader = async (substance: string) => {
  const firstLetter = substance.charAt(0);
  const substanceIndex = await substanceLoader.load(firstLetter);
  const substanceMatch = substanceIndex.find(s => s.name === substance);
  if (substanceMatch) {
    return substanceMatch.products;
  }
  return [];
};

const graphQlProductsLoader = async (substance: string) => {
  return graphQl.products.load(substance);
};

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [products, setProducts] = React.useState<IProduct[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');

  const router = useRouter();
  const {
    query: { substance: queryQS, useGraphQl: graphQlFeatureFlag },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    (async () => {
      const substanceStr = queryQS.toString();
      const loader: (
        substance: string,
      ) => Promise<IProduct[]> = graphQlFeatureFlag
        ? graphQlProductsLoader
        : azureProductsLoader;

      loader(substanceStr).then(responseData => {
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
