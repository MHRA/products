import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
import { DrugStructuredData } from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { RerouteType } from '../../model/rerouteType';
import { IDocument } from '../../model/substance';
import { docSearch, DocType } from '../../services/azure-search';
import { documents } from '../../services/documents-loader';
import Events from '../../services/events';
import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';
import { convertResults } from '../../services/results-converter';

const pageSize = 10;
const productPath = '/product';

interface IProductResult {
  name: string;
  count: number;
  documents: IDocument[];
}

interface IProductPageInfo {
  name: string;
  page: number;
  docTypes: DocType[];
}

const azureDocumentsLoader = async ({
  name,
  page,
  docTypes,
}: IProductPageInfo): Promise<IProductResult> => {
  const results = await docSearch({
    query: '',
    page,
    pageSize,
    filters: {
      docType: docTypes,
      sortOrder: 'a-z',
      productName: name,
    },
  });
  return {
    count: results.resultCount,
    name,
    documents: results.results.map(convertResults),
  };
};

const graphQlProductLoader = async ({
  name,
  page,
  docTypes,
}: IProductPageInfo): Promise<IProductResult> => {
  return documents.load({ name, page, pageSize, docTypes });
};

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
      rerouteType: rerouteTypeQS,
    },
  } = router;

  const getProduct = async (
    productPageInfo: IProductPageInfo,
  ): Promise<IProductResult> => {
    if (useGraphQl) {
      return graphQlProductLoader(productPageInfo);
    } else {
      return azureDocumentsLoader(productPageInfo);
    }
  };

  useEffect(() => {
    if (!queryQS) {
      return;
    }
    const product = queryQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;
    const docTypes = docTypesFromQueryString(docQS);
    setProductName(product);
    setPageNumber(page);
    setDocTypes(docTypes);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));
    (async () => {
      getProduct({
        name: product,
        page,
        docTypes,
      })
        .then(({ documents, count }) => {
          setDocuments(documents);
          setCount(count);
          setIsLoading(false);
        })
        .catch((e) => setErrorFetchingResults(true));

      Events.viewResultsForProduct({
        productName: product,
        pageNo: page,
        docTypes: queryStringFromDocTypes(docTypes),
      });
    })();
  }, [queryQS, pageQS, disclaimerQS, docQS]);

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
  ) => {
    const query = {
      product: productName,
      page,
    };
    if (docTypes.length > 0) {
      const docKey = 'doc';
      query[docKey] = queryStringFromDocTypes(docTypes);
    }
    router.push({
      pathname: productPath,
      query,
    });
  };

  const updateDocTypes = (updatedDocTypes: DocType[]) => {
    reroutePage(productName, 1, updatedDocTypes);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(productName, page, docTypes);
  };

  return (
    <Page
      title="Products"
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
          updateDocTypes={updateDocTypes}
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
