import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../../components/page';
import SearchResults from '../../../components/bmgf/search-results';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../../hooks';
import { IBmgfDocument } from '../../../model/substance';
import { bmgfDocSearch } from '../../../services/azure-search';
import Events from '../../../services/events';
import { parsePage } from '../../../services/querystring-interpreter';
import { convertBmgfResults } from '../../../services/results-converter';
import { searchResults } from '../../../services/search-results-loader';

const pageSize = 10;
const searchPath = '/medicine-levels-in-pregnancy';

interface ISearchResult {
  count: number;
  documents: IBmgfDocument[];
}

interface ISearchPageInfo {
  searchTerm: string;
  page: number;
}

const azureSearchPageLoader = async ({
  searchTerm,
  page,
}: ISearchPageInfo): Promise<ISearchResult> => {
  const results = await bmgfDocSearch({
    query: searchTerm,
    page,
    pageSize,
    filters: {
      sortOrder: 'a-z',
    },
  });
  return {
    count: results.resultCount,
    documents: results.results.map(convertBmgfResults),
  };
};

// const graphQlSearchPageLoader = async ({
//   searchTerm,
//   page,
// }: ISearchPageInfo): Promise<ISearchResult> => {
//   return searchResults.load({ searchTerm, page, pageSize });
// };

const App: NextPage = (props) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [documents, setDocuments] = React.useState<IBmgfDocument[]>([]);
  const [query, setQuery] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [isLoading, setIsLoading] = React.useState(true);
  const useGraphQl: boolean = false; // process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { search: queryQS, page: pageQS, doc: docQS },
  } = router;

  const getSearchResults = async (
    searchPageInfo: ISearchPageInfo,
  ): Promise<ISearchResult> => {
    // if (useGraphQl) {
    //   return graphQlSearchPageLoader(searchPageInfo);
    // } else {
    //   return azureSearchPageLoader(searchPageInfo);
    // }
    return azureSearchPageLoader(searchPageInfo);
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
      });
      setDocuments(documents);
      setCount(count);
      setIsLoading(false);
      // Events.searchForProductsMatchingKeywords({
      //   searchTerm: query,
      //   pageNo: page,
      // });
    })();
  }, [queryQS, pageQS, docQS]);

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
          reports={documents}
          showingResultsForTerm={query}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={query}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
