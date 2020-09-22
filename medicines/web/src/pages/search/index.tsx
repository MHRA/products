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
const searchPath = '/search';

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
  const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);
  const [isLoading, setIsLoading] = React.useState(true);
  const [rerouteType, setRerouteType] = React.useState(RerouteType.Other);
  const [errorFetchingResults, setErrorFetchingResults] = React.useState(false);
  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: {
      search: queryQS,
      page: pageQS,
      disclaimer: disclaimerQS,
      doc: docQS,
      rerouteType: rerouteTypeQS,
    },
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
    const docTypes = docTypesFromQueryString(docQS);
    setQuery(query);
    setPageNumber(page);
    setDocTypes(docTypes);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));

    setDocuments([]);
    setCount(0);
    setIsLoading(true);
    setErrorFetchingResults(false);

    getSearchResults({
      searchTerm: query,
      page,
      docTypes,
    })
      .then(({ documents, count }) => {
        setDocuments(documents);
        setCount(count);
        setIsLoading(false);
      })
      .catch((e) => setErrorFetchingResults(true));

    Events.searchForProductsMatchingKeywords({
      searchTerm: query,
      pageNo: page,
      docTypes: queryStringFromDocTypes(docTypes),
    });
  }, [queryQS, pageQS, disclaimerQS, docQS]);

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [props]);

  useEffect(() => {
    if (!rerouteTypeQS) {
      return;
    }
    setRerouteType(RerouteType[rerouteTypeQS.toString()]);
  }, [rerouteTypeQS]);

  const reroutePage = (
    searchTerm: string,
    page: number,
    docTypes: DocType[],
    rerouteType?: RerouteType,
  ) => {
    const query = {
      search: searchTerm,
      page,
    };
    if (docTypes.length > 0) {
      const docKey = 'doc';
      query[docKey] = queryStringFromDocTypes(docTypes);
    }
    if (rerouteType != null) {
      const rerouteTypeKey = 'rerouteType';
      query[rerouteTypeKey] = rerouteType;
    }
    router.push({
      pathname: searchPath,
      query,
    });
  };

  const updateDocTypes = (updatedDocTypes: DocType[]) => {
    if (docTypes === updatedDocTypes) return;
    reroutePage(query, 1, updatedDocTypes, RerouteType.CheckboxSelected);
  };

  const handlePageChange = async (page: number) => {
    reroutePage(query, page, docTypes);
  };

  return (
    <Page
      title="Products"
      metaTitle="Products | Search results"
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
          errorFetchingResults={errorFetchingResults}
        />
      </SearchWrapper>
    </Page>
  );
};

export default App;
