import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
import { DrugStructuredData } from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { IDocument } from '../../model/substance';
import { getBmgfDocs, IBmgfSearchResult } from '../../services/azure-search';
import { documents } from '../../services/documents-loader';
import Events from '../../services/events';
import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';

const pageSize = 10;
const productPath = '/product';

interface IProductResult {
  count: number;
  documents: IBmgfSearchResult[];
}

const azureDocumentsLoader = async (): Promise<IProductResult> => {
  const results = await getBmgfDocs('*');
  return {
    count: results.resultCount,
    documents: results.results,
  };
};

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [documents, setDocuments] = React.useState<IBmgfSearchResult[]>([]);
  const [productName, setProductName] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);
  const [isLoading, setIsLoading] = React.useState(true);

  const router = useRouter();
  const {
    query: {
      product: queryQS,
      page: pageQS,
      disclaimer: disclaimerQS,
      doc: docQS,
      useGraphQl: graphQlFeatureFlag,
    },
  } = router;

  useEffect(() => {
    setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));
    (async () => {
      const { documents, count } = await azureDocumentsLoader();
      setDocuments(documents);
      setCount(count);
      setIsLoading(false);
    })();
  }, [queryQS, pageQS, disclaimerQS, docQS]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  // const handlePageChange = async (page: number) => {
  //   reroutePage(productName, page, docTypes);
  // };
  const getReportName = path => {
    return path.match(/(?<=bmgf-docs\/).*(?=\/)/);
  };

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        {documents.map((document, i) => (
          <div key={i}>
            <a href={`/bmgf/${getReportName(document.metadata_storage_path)}`}>
              {decodeURIComponent(
                getReportName(document.metadata_storage_path),
              )}
            </a>
          </div>
        ))}
      </SearchWrapper>
    </Page>
  );
};

export default App;
