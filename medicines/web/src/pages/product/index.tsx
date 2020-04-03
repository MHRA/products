import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
import { DrugStructuredData } from '../../components/structured-data';
import { useLocalStorage } from '../../hooks';
import { docSearch, DocType } from '../../services/azure-search';
import Events from '../../services/events';
import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';
import { convertResults, IDocument } from '../../services/results-converter';

const pageSize = 10;
const productPath = '/product';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<IDocument[]>([]);
  const [productName, setProductName] = React.useState('');
  const [count, setCount] = React.useState();
  const [pageNumber, setPageNumber] = React.useState();
  const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState();
  const [isLoading, setIsLoading] = React.useState(true);

  const router = useRouter();
  const {
    query: {
      product: queryQS,
      page: pageQS,
      disclaimer: disclaimerQS,
      doc: docQS,
    },
  } = router;

  const getResults = async (
    product: string,
    page: number,
    docTypes: DocType[],
  ) => {
    return docSearch({
      query: '',
      page,
      pageSize,
      filters: {
        docType: docTypes,
        sortOrder: 'a-z',
        productName: product,
      },
    });
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
      const results = await getResults(product, page, docTypes);
      setResults(results.results.map(convertResults));
      setCount(results.resultCount);
      setIsLoading(false);
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

  const reroutePage = (
    productName: string,
    page: number,
    docTypes: DocType[],
  ) => {
    const query = {
      productFilter: productName,
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

  const handleToggleDocType = async (docTypeToToggle: DocType) => {
    const enabledDocTypes = Array.from(docTypes);
    if (enabledDocTypes.includes(docTypeToToggle)) {
      const docTypeIndex = enabledDocTypes.indexOf(docTypeToToggle);
      enabledDocTypes.splice(docTypeIndex, 1);
    } else {
      enabledDocTypes.push(docTypeToToggle);
    }
    reroutePage(productName, 1, enabledDocTypes);
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
          drugs={results}
          showingResultsForTerm={productName}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={productName}
          disclaimerAgree={disclaimerAgree}
          docTypes={docTypes}
          handleDocTypeCheckbox={handleToggleDocType}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
        />
        <DrugStructuredData drugName={productName} />
      </SearchWrapper>
    </Page>
  );
};

export default App;
