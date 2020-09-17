import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../components/page';
import SearchResults from '../../components/search-results';
import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import { RerouteType } from '../../model/rerouteType';
import { IDocument } from '../../model/substance';
import { docSearch, DocType } from '../../services/azure-search';
import Events from '../../services/events';
import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';
import { convertResults } from '../../services/results-converter';
import { searchResults } from '../../services/search-results-loader';

const pageSize = 10;
const searchPath = '/medicine-levels-in-pregnancy';

interface ISearchResult {
  count: number;
  documents: IDocument[];
}

interface ISearchPageInfo {
  searchTerm: string;
  page: number;
  docTypes: DocType[];
}

const azureSearchPageLoader = async ({
  searchTerm,
  page,
  docTypes,
}: ISearchPageInfo): Promise<ISearchResult> => {
  const results = await docSearch({
    query: searchTerm,
    page,
    pageSize,
    filters: {
      docType: docTypes,
      sortOrder: 'a-z',
    },
  });
  return {
    count: results.resultCount,
    documents: results.results.map(convertResults),
  };
};

const graphQlSearchPageLoader = async ({
  searchTerm,
  page,
  docTypes,
}: ISearchPageInfo): Promise<ISearchResult> => {
  return searchResults.load({ searchTerm, page, pageSize, docTypes });
};

const App: NextPage = (props) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [documents, setDocuments] = React.useState<IDocument[]>([]);
  const [query, setQuery] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [isLoading, setIsLoading] = React.useState(true);
  const useGraphQl: boolean = false; //process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { search: queryQS, page: pageQS, doc: docQS },
  } = router;

  const getSearchResults = async (
    searchPageInfo: ISearchPageInfo,
  ): Promise<ISearchResult> => {
    if (useGraphQl) {
      return graphQlSearchPageLoader(searchPageInfo);
    } else {
      return azureSearchPageLoader(searchPageInfo);
    }
  };

  useEffect(() => {
    setIsLoading(true);
    if (!queryQS) {
      return;
    }
    const query = queryQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;

    setQuery(query);
    setPageNumber(page);

    (async () => {
      const { documents, count } = await getSearchResults({
        searchTerm: query,
        page,
        docTypes,
      });
      setDocuments(documents);
      setCount(count);
      setIsLoading(false);
      Events.searchForProductsMatchingKeywords({
        searchTerm: query,
        pageNo: page,
        docTypes: queryStringFromDocTypes(docTypes),
      });
    })();
  }, [queryQS, pageQS, disclaimerQS, docQS]);

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [props]);

  const reroutePage = (searchTerm: string, page: number) => {
    const query = {
      search: searchTerm,
      page,
    };
    router.push({
      pathname: searchPath,
      query,
    });
  };

  const handlePageChange = async (page: number) => {
    reroutePage(query, page);
  };

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue={query}>
        <SearchResults
          drugs={documents}
          showingResultsForTerm={query}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={query}
          disclaimerAgree={disclaimerAgree}
          docTypes={docTypes}
          updateDocTypes={updateDocTypes}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
          rerouteType={rerouteType}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
