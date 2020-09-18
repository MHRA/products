import React from 'react';
import glob from 'glob';
import parse5 from 'parse5';
import fetch, { Response } from 'node-fetch';

import Page from '../../../components/page';
import {
  getReportList,
  getReportContent,
} from '../../../services/bmgf-reports-fetcher';

import { useLocalStorage } from '../../../hooks';

const Report = ({ htmlBody }) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  return (
    <Page
      title="Products"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <div
        dangerouslySetInnerHTML={{
          __html: htmlBody,
        }}
      ></div>
    </Page>
  );
};

export default Report;

const updateImageTag = (imageNode, prefix) => {
  for (let i = 0; i < imageNode.attrs.length; i++) {
    if (imageNode.attrs[i].name === 'src') {
      let imageName = imageNode.attrs[i].value.split('/').pop();
      imageNode.attrs[i].value = `${prefix}${imageName}`;
      return imageNode;
    }
  }
  return imageNode;
};

const recurseNodes = (node, prefix) => {
  if (node.tagName && node.tagName === 'img') {
    node = updateImageTag(node, prefix);
  }
  if (!node.childNodes) {
    return node;
  }
  for (let i = 0; i < node.childNodes.length; i++) {
    node.childNodes[i] = recurseNodes(node.childNodes[i], prefix);
  }
  return node;
};

const getHtmlBody = (htmlDoc) => {
  let html = htmlDoc.childNodes[0];
  for (let i = 0; i < html.childNodes.length; i++) {
    if (html.childNodes[i].tagName === 'body') {
      return html.childNodes[i];
    }
  }
};

export const getStaticProps = async (context) => {
  const reportName = context.params.report;
  const containerUrl = `https://${process.env.AZURE_STORAGE_ACCOUNT}.blob.core.windows.net/bmgf-docs`;

  let reportContent = await getReportContent(reportName, containerUrl);

  const htmlDocument = parse5.parse(reportContent, { scriptingEnabled: false });
  let htmlBody = getHtmlBody(htmlDocument);

  let assetsUrl = `${containerUrl}/${reportName}/assets/`;
  htmlBody = recurseNodes(htmlBody, assetsUrl);

  return {
    props: {
      htmlBody: parse5.serialize(htmlBody),
    },
  };
};

export const getStaticPaths = async () => {
  const containerUrl = `https://${process.env.AZURE_STORAGE_ACCOUNT}.blob.core.windows.net/bmgf-docs`;
  let reports = await getReportList(containerUrl).then((reportPaths) =>
    reportPaths.map(
      (reportPath) =>
        `/medicine-levels-in-pregnancy/reports/${reportPath.split('/')[0]}`,
    ),
  );

  return {
    paths: reports,
    fallback: false,
  };
};
