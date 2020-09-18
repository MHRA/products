import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';

import Page from '../../../components/page';
import SearchResults from '../../../components/bmgf/search-results';
import SearchWrapper from '../../../components/bmgf/search-wrapper';
import { DrugStructuredData } from '../../../components/structured-data';
import { useLocalStorage } from '../../../hooks';
import { IBmgfDocument } from '../../../model/substance';
import { bmgfDocSearch, DocType } from '../../../services/azure-search';
import { documents } from '../../../services/documents-loader';
import Events from '../../../services/events';
import { parsePage } from '../../../services/querystring-interpreter';
import { convertBmgfResults } from '../../../services/results-converter';

const pageSize = 10;
const substancePath = '/substance';

interface ISubstanceResult {
  name: string;
  count: number;
  documents: IBmgfDocument[];
}

interface ISubstancePageInfo {
  name: string;
  page: number;
}

const azureDocumentsLoader = async ({
  name,
  page,
}: ISubstancePageInfo): Promise<ISubstanceResult> => {
  const results = await bmgfDocSearch({
    query: '',
    page,
    pageSize,
    filters: {
      sortOrder: 'a-z',
      substanceName: name,
    },
  });
  return {
    count: results.resultCount,
    name,
    documents: results.results.map(convertBmgfResults),
  };
};

// const graphQlProductLoader = async ({
//   name,
//   page,
//   docTypes,
// }: ISubstancePageInfo): Promise<ISubstanceResult> => {
//   return documents.load({ name, page, pageSize });
// };

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [documents, setDocuments] = React.useState<IBmgfDocument[]>([]);
  const [substanceName, setSubstanceName] = React.useState('');
  const [count, setCount] = React.useState(0);
  const [pageNumber, setPageNumber] = React.useState(1);
  const [isLoading, setIsLoading] = React.useState(true);

  const useGraphQl: boolean = process.env.USE_GRAPHQL === 'true';

  const router = useRouter();
  const {
    query: { substance: substanceQS, page: pageQS },
  } = router;

  const getSubstance = async (
    substancePageInfo: ISubstancePageInfo,
  ): Promise<ISubstanceResult> => {
    // if (useGraphQl) {
    //   return graphQlProductLoader(productPageInfo);
    // } else {
    //   return azureDocumentsLoader(productPageInfo);
    // }
    return azureDocumentsLoader(substancePageInfo);
  };

  useEffect(() => {
    if (!substanceQS) {
      return;
    }
    const substance = substanceQS.toString();
    const page = pageQS ? parsePage(pageQS) : 1;
    setSubstanceName(substance);
    setPageNumber(page);

    (async () => {
      const { documents, count } = await getSubstance({
        name: substance,
        page,
      });
      setDocuments(documents);
      setCount(count);
      setIsLoading(false);
      // Events.viewResultsForProduct({
      //   productName: product,
      //   pageNo: page,
      //   docTypes: queryStringFromDocTypes(docTypes),
      // });
    })();
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
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <SearchResults
          reports={documents}
          showingResultsForTerm={substanceName}
          resultCount={count}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={substanceName}
          handlePageChange={handlePageChange}
          isLoading={isLoading}
        />
        <DrugStructuredData drugName={substanceName} />
      </SearchWrapper>
    </Page>
  );
};

export default App;
