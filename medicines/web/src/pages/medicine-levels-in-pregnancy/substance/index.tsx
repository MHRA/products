import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import { BmgfPage } from '../../../components/page';
import SearchResults from '../../../components/bmgf/search-results';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { DrugStructuredData } from '../../../components/structured-data';
import { useLocalStorage } from '../../../hooks';
import { IBmgfReport } from '../../../model/document';
import { getLoader } from '../../../services/loaders/medicine-levels-in-pregnancy/substance-loader';
import Events from '../../../services/events';
import { parsePage } from '../../../services/querystring-interpreter';

const pageSize = 10;
const substancePath = '/medicine-levels-in-pregnancy/substance';

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [reports, setReports] = React.useState<IBmgfReport[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [isLoading, setIsLoading] = React.useState(true);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);

  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { substance: substanceQS, page: pageQS },
  } = router;

  useEffect(() => {
    if (!substanceQS) {
      return;
    }
    const substance = substanceQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;
    setSubstanceName(substance);
    setPageNumber(page);
    setReports([]);
    setCount(0);
    setIsLoading(true);
    setErrorFetchingResults(false);

    getLoader(useGraphQl)
      .load({
        name: substance,
        page,
        pageSize,
      })
      .then(({ reports, count }) => {
        setReports(reports);
        setCount(count);
        setIsLoading(false);
      })
      .catch((e) => setErrorFetchingResults(true));

    Events.viewPbpkResultsForSubstance({
      substance,
      pageNo: page,
    });
  }, [substanceQS, pageQS]);

  useEffect(() => {
    if (window) {
      window.scrollTo(0, 0);
    }
  }, []);

  const reroutePage = (substanceName: string, page: number) => {
    const query = {
      substance: substanceName,
      page,
    };
    router.push({
      pathname: substancePath,
      query,
    });
  };

  const handlePageChange = async (page: number) => {
    reroutePage(substanceName, page);
  };

  return (
    <BmgfPage
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy | Substance results"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <SearchResults
          reports={reports}
          showingResultsForTerm={substanceName}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={substanceName}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
          errorFetchingResults={errorFetchingResults}
        />
        <DrugStructuredData drugName={substanceName} />
      </SearchWrapper>
    </BmgfPage>
  );
};

export default App;
