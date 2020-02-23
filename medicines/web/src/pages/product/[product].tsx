import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import Page from '../../components/page';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import { NextPage, NextPageContext } from 'next';
import { convertResults, IDocument } from '../../services/results-converter';
import {
  docTypesFromQueryString,
  parsePage,
  queryStringFromDocTypes,
  parseDisclaimerAgree,
} from '../../services/querystring-interpreter';
import { docSearch, DocType } from '../../services/azure-search';
import Events from '../../services/events';
import SearchResults from '../../components/search-results';

interface IAppProps {
  results: IDocument[];
  count: number;
  productName: string;
  page: number;
  docTypes: DocType[];
  disclaimerAgree: boolean;
}

const pageSize = 2;
const productPath = '/product/';

const App: NextPage<IAppProps> = props => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const router = useRouter();

  useEffect(() => {
    if (!props.productName) {
      router.push('/');
    }
  }, [props.productName]);

  const reroutePage = (
    productName: string,
    page: number,
    docTypes: DocType[],
  ) => {
    let query = {
      page: page,
    };
    if (docTypes.length > 0) {
      query['doc'] = queryStringFromDocTypes(docTypes);
    }
    router.push({
      pathname: `${productPath}${encodeURIComponent(productName)}`,
      query,
    });
  };

  const handleToggleDocType = async (docTypeToToggle: DocType) => {
    const enabledDocTypes = Array.from(props.docTypes);
    if (enabledDocTypes.includes(docTypeToToggle)) {
      const docTypeIndex = enabledDocTypes.indexOf(docTypeToToggle);
      enabledDocTypes.splice(docTypeIndex, 1);
    } else {
      enabledDocTypes.push(docTypeToToggle);
    }
    reroutePage(props.productName, 1, enabledDocTypes);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(props.productName, page, props.docTypes);
  };

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <SearchResults
          drugs={props.results}
          showingResultsForTerm={props.productName}
          resultCount={props.count}
          page={props.page}
          pageSize={pageSize}
          searchTerm={props.productName}
          disclaimerAgree={props.disclaimerAgree}
          docTypes={props.docTypes}
          handleDocTypeCheckbox={handleToggleDocType}
          handlePageChange={handlePageChange}
        />
      </SearchWrapper>
    </Page>
  );
};

App.getInitialProps = async (context: NextPageContext): Promise<IAppProps> => {
  const {
    query: { product, page, disclaimer, doc: queryDocFilter },
  } = context;
  const docTypes = docTypesFromQueryString(queryDocFilter);
  const parsedPage = parsePage(page);
  let results = [];
  let count = 0;
  let productName = '';
  if (product) {
    productName = decodeURIComponent(product.toString());
    const searchResults = await docSearch({
      query: '',
      page: parsedPage,
      pageSize,
      filters: {
        docType: docTypes,
        sortOrder: 'a-z',
        productName,
      },
    });
    results = searchResults.results.map(convertResults);
    count = searchResults.resultCount;
  }

  return {
    results,
    count,
    productName,
    page: parsedPage,
    docTypes,
    disclaimerAgree: parseDisclaimerAgree(disclaimer),
  };
};

export default App;
