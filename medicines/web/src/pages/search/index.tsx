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
const searchPath = '/search';

const App: NextPage = props => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [results, setResults] = React.useState<IDocument[]>([]);
  const [query, setQuery] = React.useState('');
  const [count, setCount] = React.useState();
  const [pageNumber, setPageNumber] = React.useState();
  const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState();

  const router = useRouter();
  const {
    query: { search, page, disclaimer, doc },
  } = router;

  const setPageValues = async (
    search: string | string[],
    page: string | string[],
    disclaimer: string | string[],
    doc: string | string[],
  ) => {
    const docTypes = docTypesFromQueryString(doc);
    const parsedPage = page ? parsePage(page) : 1;
    const searchStr = search.toString();
    const results = await docSearch({
      query: searchStr,
      page: parsedPage,
      pageSize,
      filters: {
        docType: docTypes,
        sortOrder: 'a-z',
      },
    });

    setQuery(searchStr);
    setPageNumber(parsedPage);
    setDocTypes(docTypes);
    setResults(results.results.map(convertResults));
    setCount(results.resultCount);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimer));
    Events.searchForProductsMatchingKeywords({
      searchTerm: searchStr,
      pageNo: parsedPage,
      docTypes: queryStringFromDocTypes(docTypes),
    });
  };

  useEffect(() => {
    if (!search) {
      return;
    }
    (async () => {
      await setPageValues(search, page, disclaimer, doc);
    })();
  }, [search, page, disclaimer, doc]);

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [props]);

  const reroutePage = (
    searchTerm: string,
    page: number,
    docTypes: DocType[],
  ) => {
    const query = {
      search: searchTerm,
      page,
    };
    if (docTypes.length > 0) {
      const docKey = 'doc';
      query[docKey] = queryStringFromDocTypes(docTypes);
    }
    router.push({
      pathname: searchPath,
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
    reroutePage(query, 1, enabledDocTypes);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(query, page, docTypes);
  };

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue={query}>
        <SearchResults
          drugs={results}
          showingResultsForTerm={query}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={query}
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
