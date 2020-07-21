import { NextPage } from 'next';
import { useRouter } from 'next/router';
import React, { useEffect } from 'react';
import matter from 'gray-matter';
import glob from 'node-glob';
import ReactMarkdown, { uriTransformer } from 'react-markdown';
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
// import { IDocument } from '../../model/substance';
// import { getBmgfDocs } from '../../services/azure-search';
// import { documents } from '../../services/documents-loader';
// import Events from '../../services/events';
import {
  docTypesFromQueryString,
  parseDisclaimerAgree,
  parsePage,
  queryStringFromDocTypes,
} from '../../services/querystring-interpreter';
import { report } from 'process';
// import { convertResults } from '../../services/results-converter';

// // const pageSize = 10;
// // const productPath = '/product';

// // interface IProductResult {
// //   name: string;
// //   count: number;
// //   documents: IDocument[];
// // }

// // interface IProductPageInfo {
// //   name: string;
// //   page: number;
// //   docTypes: DocType[];
// // }

// interface IProductResult {
//   count: number;
//   documents: IBmgfSearchResult[];
// }

// const azureDocumentsLoader = async (
//   report: string,
// ): Promise<IProductResult> => {
//   const results = await getBmgfDocs(report);
//   return {
//     count: results.resultCount,
//     documents: results.results,
//   };
// };

// // const graphQlProductLoader = async ({
// //   name,
// //   page,
// //   docTypes,
// // }: IProductPageInfo): Promise<IProductResult> => {
// //   return documents.load({ name, page, pageSize, docTypes });
// // };

// const App: NextPage = () => {
//   const [storageAllowed, setStorageAllowed] = useLocalStorage(
//     'allowStorage',
//     false,
//   );
//   const [content, setContent] = React.useState();
//   // const [productName, setProductName] = React.useState('');
//   // const [count, setCount] = React.useState(0);
//   // const [pageNumber, setPageNumber] = React.useState(1);
//   // const [docTypes, setDocTypes] = React.useState<DocType[]>([]);
//   const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);
//   // const [isLoading, setIsLoading] = React.useState(true);

//   const router = useRouter();
//   const {
//     query: { report, disclaimer: disclaimerQS },
//   } = router;
//   console.log(report);

//   useEffect(() => {
//     if (!report || !report.length) {
//       return;
//     }
//     setDisclaimerAgree(parseDisclaimerAgree(disclaimerQS));
//     (async () => {
//       if (document) {
//         getMarkdownDoc('/about.md')
//           .then(function(response) {
//             return response.text();
//           })
//           .then(function(data) {
//             setContent(marked(data));
//           });
//       }
//     })();
//   }, [report, disclaimerQS]);

//   return (
//     <Page
//       title="Products"
//       storageAllowed={storageAllowed}
//       setStorageAllowed={setStorageAllowed}
//     >
//       <article dangerouslySetInnerHTML={{ __html: content }}></article>
//     </Page>
//   );
// };

// export default App;

const Report = ({ markdownBody }) => {
  const allowNode = (node, index, parent) => {
    return node.type !== 'script';
  };

  const transformUri = uri => {
    // TODO: add processing here
    let rewrittenUri = `/bmgf/images/${uri}`;
    // calls through to default transformer to perform XSS filter
    return uriTransformer(rewrittenUri);
  };

  return (
    <div>
      <ReactMarkdown
        escapeHtml={false}
        source={markdownBody}
        allowNode={allowNode}
        transformLinkUri={transformUri}
      />
    </div>
  );
};

export default Report;

export async function getStaticProps(context) {
  const reportName = context.params;
  // @ts-ignore
  const reportContent = await import('../../content/about.md');
  console.log(reportContent);
  return {
    props: {
      markdownBody: reportContent.default,
    },
  };
}

export async function getStaticPaths() {
  //get all .md files in the posts dir
  // glob.sync('../../content/**/*.md');
  const reports = ['../../content/about.md'];

  //remove path and extension to leave filename only
  const reportNames = reports.map(file =>
    file
      .split('/')[1]
      .replace(/ /g, '-')
      .slice(0, -3)
      .trim(),
  );

  // create paths with `slug` param
  const paths = reportNames.map(report => {
    return {
      params: {
        report: `about`,
      },
    };
  });

  return {
    paths,
    fallback: false,
  };
}
