import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import { BmgfPage } from '../../../components/page';
import SearchResults from '../../../components/bmgf/search-results';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../../hooks';
import { IBmgfReport } from '../../../model/document';
import Events from '../../../services/events';
import { parsePage } from '../../../services/querystring-interpreter';
import { getLoader } from '../../../services/loaders/medicine-levels-in-pregnancy/search-results-loader';

const pageSize = 10;
const searchPath = '/medicine-levels-in-pregnancy/search';

const App: NextPage = (props) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [reports, setReports] = React.useState<IBmgfReport[]>([]);
  const [query, setQuery] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [isLoading, setIsLoading] = React.useState(true);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);
  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { search: queryQS, page: pageQS, doc: docQS },
  } = router;

  useEffect(() => {
    setIsLoading(true);
    if (!queryQS) {
      return;
    }
    const query = queryQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;

    setQuery(query);
    setPageNumber(page);

    setReports([]);
    setCount(0);
    setErrorFetchingResults(false);

    getLoader(useGraphQl)
      .load({
        searchTerm: query,
        page,
        pageSize,
      })
      .then(({ reports, count }) => {
        setReports(reports);
        setCount(count);
        setIsLoading(false);
      })
      .catch((e) => setErrorFetchingResults(true));

    Events.searchForPbpkReportsMatchingKeywords({
      searchTerm: query,
      pageNo: page,
    });
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
    <BmgfPage
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy | Search results"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue={query}>
        <SearchResults
          reports={reports}
          showingResultsForTerm={query}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={query}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
          errorFetchingResults={errorFetchingResults}
        />
      </SearchWrapper>
    </BmgfPage>
  );
};

export default App;
