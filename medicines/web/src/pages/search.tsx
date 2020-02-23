import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import Page from '../components/page';
import SearchWrapper from '../components/search-wrapper';
import { useLocalStorage } from '../hooks';
import { NextPage, NextPageContext } from 'next';
import { convertResults, IDocument } from '../services/results-converter';
import {
  docTypesFromQueryString,
  parsePage,
  queryStringFromDocTypes,
  parseDisclaimerAgree,
} from '../services/querystring-interpreter';
import { docSearch, DocType } from '../services/azure-search';
import Events from '../services/events';
import SearchResults from '../components/search-results';

interface IAppProps {
  results: IDocument[];
  count: number;
  query: string;
  page: number;
  docTypes: DocType[];
  disclaimerAgree: boolean;
}

const pageSize = 10;
const searchPath = '/search';

const App: NextPage<IAppProps> = props => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const router = useRouter();

  useEffect(() => {
    if (!props.query) {
      router.push('/');
    }
  }, [props.query]);

  const reroutePage = (
    searchTerm: string,
    page: number,
    docTypes: DocType[],
  ) => {
    let query = {
      q: encodeURIComponent(searchTerm),
      page: page,
    };
    if (docTypes.length > 0) {
      query['doc'] = queryStringFromDocTypes(docTypes);
    }
    router.push({
      pathname: searchPath,
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
    reroutePage(props.query, 1, enabledDocTypes);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(props.query, page, props.docTypes);
  };

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue={props.query}>
        <SearchResults
          drugs={props.results}
          showingResultsForTerm={props.query}
          resultCount={props.count}
          page={props.page}
          pageSize={pageSize}
          searchTerm={props.query}
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
    query: { q, page, disclaimer, doc: queryDocFilter },
  } = context;
  const docTypes = docTypesFromQueryString(queryDocFilter);
  const parsedPage = parsePage(page);
  let results = [];
  let count = 0;
  let query = '';
  if (q) {
    query = decodeURIComponent(q.toString());
    const searchResults = await docSearch({
      query,
      page: parsedPage,
      pageSize,
      filters: {
        docType: docTypes,
        sortOrder: 'a-z',
      },
    });
    results = searchResults.results.map(convertResults);
    count = searchResults.resultCount;
  }

  return {
    results,
    count,
    query,
    page: parsedPage,
    docTypes,
    disclaimerAgree: parseDisclaimerAgree(disclaimer),
  };
};

export default App;
