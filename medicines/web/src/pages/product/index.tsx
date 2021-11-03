import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
import { DrugStructuredData } from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { RerouteType } from '../../model/rerouteType';
import { IDocument } from '../../model/document';
import {
  DocType,
  TerritoryType,
  SearchType,
} from '../../services/azure-search';
import { getLoader } from '../../services/loaders/products/product-loader';
import Events from '../../services/events';
import {
  docTypesFromQueryString,
  territoryTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromTypes,
} from '../../services/querystring-interpreter';

const pageSize = 10;
const productPath = '/product';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [documents, setDocuments] = React.useState<IDocument[]>([]);
  const [productName, setProductName] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
  const [territoryTypes, setTerritoryTypes] = React.useState<TerritoryType[]>(
    [],
  );
  const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);
  const [isLoading, setIsLoading] = React.useState(true);
  const [rerouteType, setRerouteType] = React.useState(RerouteType.Other);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);
  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: {
      product: queryQS,
      page: pageQS,
      disclaimer: disclaimerQS,
      doc: docQS,
      ter: territoryQS,
      rerouteType: rerouteTypeQS,
    },
  } = router;

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    const product = queryQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;
    const docTypes = docTypesFromQueryString(docQS);
    const territoryTypes = territoryTypesFromQueryString(territoryQS);
    setProductName(product);
    setPageNumber(page);
    setDocTypes(docTypes);
    setTerritoryTypes(territoryTypes);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));

    setDocuments([]);
    setCount(0);
    setIsLoading(true);
    setErrorFetchingResults(false);

    getLoader(useGraphQl)
      .load({ name: product, page, pageSize, docTypes, territoryTypes })
      .then(({ documents, count }) => {
        setDocuments(documents);
        setCount(count);
        setIsLoading(false);
      })
      .catch((e) => setErrorFetchingResults(true));

    Events.viewResultsForProduct({
      productName: product,
      pageNo: page,
      docTypes: queryStringFromTypes(docTypes),
    });
  }, [queryQS, pageQS, disclaimerQS, docQS, territoryQS]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  useEffect(() => {
    if (!rerouteTypeQS) {
      return;
    }
    setRerouteType(RerouteType[rerouteTypeQS.toString()]);
  }, [rerouteTypeQS]);

  const reroutePage = (
    productName: string,
    page: number,
    docTypes: DocType[],
    territoryTypes: TerritoryType[],
  ) => {
    const query = {
      product: productName,
      page,
    };
    if (docTypes.length > 0) {
      query[SearchType.Doc] = queryStringFromTypes(docTypes);
    }
    if (territoryTypes.length > 0) {
      query[SearchType.Territory] = queryStringFromTypes(territoryTypes);
    }
    router.push({
      pathname: productPath,
      query,
    });
  };

  const updatePageFilters = (
    updatedDocTypes: DocType[],
    updatedTerritoryTypes: TerritoryType[],
  ) => {
    reroutePage(productName, 1, updatedDocTypes, updatedTerritoryTypes);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(productName, page, docTypes, territoryTypes);
  };

  return (
    <Page
      title="Products"
      metaTitle="Products | Product results"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <SearchResults
          drugs={documents}
          showingResultsForTerm={productName}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={productName}
          disclaimerAgree={disclaimerAgree}
          docTypes={docTypes}
          territoryTypes={territoryTypes}
          updatePageFilters={updatePageFilters}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
          rerouteType={rerouteType}
          errorFetchingResults={errorFetchingResults}
        />
        <DrugStructuredData drugName={productName} />
      </SearchWrapper>
    </Page>
  );
};

export default App;
