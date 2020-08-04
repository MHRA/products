import React, { useEffect } from 'react';
import glob from 'node-glob';
import ReactMarkdown, { uriTransformer } from 'react-markdown';

const Report = ({ markdownBody }) => {
  const allowNode = node => {
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
  console.log(reportName);
  // @ts-ignore
  const reportContent = await import(`../../content/${reportName}/index.md`);
  console.log(reportContent);
  return {
    props: {
      markdownBody: reportContent.default,
    },
  };
}

export async function getStaticPaths() {
  //get all .md files in the posts dir
  glob.sync('../../content/**/*.md');
  // const reports = ['../../content/about.md'];

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
