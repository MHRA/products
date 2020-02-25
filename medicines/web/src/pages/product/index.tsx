import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
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
  const [productName, setProductName] = React.useState();
  const [count, setCount] = React.useState();
  const [pageNumber, setPageNumber] = React.useState();
  const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState();

  const router = useRouter();
  const {
    query: { product, page, disclaimer, doc },
  } = router;

  const setPageValues = async (
    product: string | string[],
    page: string | string[],
    disclaimer: string | string[],
    doc: string | string[],
  ) => {
    const docTypes = docTypesFromQueryString(doc);
    const parsedPage = page ? parsePage(page) : 1;
    const productStr = product.toString();
    const results = await docSearch({
      query: '',
      page: parsedPage,
      pageSize,
      filters: {
        docType: docTypes,
        sortOrder: 'a-z',
        productName: productStr,
      },
    });

    setProductName(productStr);
    setPageNumber(parsedPage);
    setDocTypes(docTypes);
    setResults(results.results.map(convertResults));
    setCount(results.resultCount);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimer));
  };

  useEffect(() => {
    if (!product) {
      return;
    }
    (async () => {
      await setPageValues(product, page, disclaimer, doc);
    })();
  }, [product, page, disclaimer, doc]);

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
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
