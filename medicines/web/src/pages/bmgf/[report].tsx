import { NextPage } from 'next';
import dynamic from 'next/dynamic';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import marked from 'marked';
import Page from '../../components/page';
// import SearchResults from '../../components/search-results';
// import SearchWrapper from '../../components/search-wrapper';
import { useLocalStorage } from '../../hooks';
import {
  getBmgfDocs,
  IBmgfSearchResult,
  getMarkdownDoc,
} from '../../services/azure-search';

import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';

interface IProductResult {
  count: number;
  documents: IBmgfSearchResult[];
}

const App: NextPage = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [content, setContent] = React.useState();
  const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);
  let ReportElement;

  const router = useRouter();
  const {
    query: { report, disclaimer: disclaimerQS },
  } = router;
  console.log(report);

  useEffect(() => {
    if (!report || !report.length) {
      return;
    }
    console.log(report);
    setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));
    // @ts-ignore
    const ReportElement = dynamic(() => import(`../../content/about`), {
      ssr: false,
    });
    // setContent(con);
  }, [report, disclaimerQS]);

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <ReportElement></ReportElement>
      {/* <article dangerouslySetInnerHTML={{ __html: content }}></article> */}
    </Page>
  );
};

export default App;
