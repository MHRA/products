import React from 'react';
import parse5 from 'parse5';
import styled from 'styled-components';

import Page from '../../../components/page';
import {
  getReportList,
  getReportUrl,
  getReportHtmlContent,
} from '../../../services/bmgf-reports-fetcher';

import { useLocalStorage } from '../../../hooks';
import { mhraWhite, primaryColor, mhra70 } from '../../../styles/colors';

const DownloadButton = styled.a`
  display: block;
  cursor: pointer;
  color: ${mhraWhite};
  background-color: ${primaryColor};
  align-self: flex-end;
  max-width: 50%;
  border-radius: 0.375rem;
  text-decoration: none;
  appearance: none;
  border: solid 1px ${mhra70};
  padding: 0.5em 1em;
  margin-top: 1em;

  &:hover:enabled {
    background-color: ${mhra70};
  }
`;

const Report = ({ reportName, htmlBody, pdfUrl }) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  return (
    <Page
      title={reportName}
      metaTitle={reportName}
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <div>
        <DownloadButton href={pdfUrl}>Download report (PDF)</DownloadButton>
      </div>
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

const removeStyleAttribute = (node) => {
  for (let i = 0; i < node.attrs.length; i++) {
    if (node.attrs[i].name === 'style') {
      node.attrs[i].value = '';
      return node;
    }
  }
  return node;
};

const recurseNodes = (node, prefix) => {
  if (node.tagName && node.tagName === 'img') {
    node = updateImageTag(node, prefix);
  }
  if (node.attrs) {
    node = removeStyleAttribute(node);
  }
  if (node.tagName === 'h1') {
    return;
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

  const reportUrl = await getReportUrl(reportName);
  const reportContent = await getReportHtmlContent(reportUrl);

  const htmlDocument = parse5.parse(reportContent, { scriptingEnabled: false });
  let htmlBody = getHtmlBody(htmlDocument);

  const assetsUrl = `${reportUrl}/assets/`;
  htmlBody = recurseNodes(htmlBody, assetsUrl);

  const pdfUrl = `${reportUrl}.pdf`;

  return {
    props: {
      htmlBody: parse5.serialize(htmlBody),
      reportName,
      pdfUrl,
    },
  };
};

export const getStaticPaths = async () => {
  const reports = await getReportList().then((reportPaths) =>
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
