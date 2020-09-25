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
import { mhraWhite, primaryColor, mhra70, mhra } from '../../../styles/colors';

const ReportBody = styled.div`
  padding: 0 10px 0 20px;
  table {
    border-collapse: collapse;

    p {
      margin: 0;
    }
  }

  table,
  th,
  td {
    border: 1px solid black;
  }

  td {
    padding: 10px;
  }
`;

const DownloadButtonContainer = styled.section`
  text-align: right;

  & > a {
    appearance: none;
    color: ${mhraWhite};
    background-color: ${primaryColor};
    border-radius: 5px;
    border: 1px solid ${mhra};
    padding: 8px 16px;
    cursor: pointer;
    text-decoration: none;
    display: inline-block;
    margin: 12px 0 4px;

    &:hover:enabled {
      background-color: ${mhra70};
    }
  }
`;

const AccessibleHeading = styled.h2`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
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
      <ReportBody>
        <DownloadButtonContainer>
          <AccessibleHeading>Download PDF version of report</AccessibleHeading>
          <a href={encodeURI(pdfUrl)} download={reportName}>
            Download report (PDF)
          </a>
        </DownloadButtonContainer>
        <section>
          <AccessibleHeading>Report content</AccessibleHeading>
          <div
            dangerouslySetInnerHTML={{
              __html: htmlBody,
            }}
          ></div>
        </section>
      </ReportBody>
    </Page>
  );
};

export default Report;

const updateImageTag = (imageNode, prefix) => {
  for (let i = 0; i < imageNode.attrs.length; i++) {
    if (imageNode.attrs[i].name === 'src') {
      let imageName = imageNode.attrs[i].value.split('/').pop();
      imageNode.attrs[i].value = encodeURI(`${prefix}${imageName}`);
    } else if (imageNode.attrs[i].name === 'v:shapes') {
      imageNode.attrs.splice(i, 1);
      i--;
    }
  }
  return imageNode;
};

const updateAnchorNameToId = (node) => {
  for (let i = 0; i < node.attrs.length; i++) {
    if (node.attrs[i].name === 'name') {
      node.attrs[i].name = 'id';
      return node;
    }
  }
  return node;
};

const removeUnwantedTableAttributes = (node) => {
  if (!node.attrs) {
    return node;
  }

  const unwantedAttributes = [
    'style',
    'v:shapes',
    'cellspacing',
    'cellpadding',
    'border',
    'width',
    'valign',
  ];

  for (let i = 0; i < node.attrs.length; i++) {
    if (unwantedAttributes.includes(node.attrs[i].name)) {
      node.attrs.splice(i, 1);
      i--;
    }
  }

  return node;
};

const removeUnwantedAttributes = (node) => {
  if (!node.attrs) {
    return node;
  }

  const unwantedAttributes = ['style', 'align'];
  for (let i = 0; i < node.attrs.length; i++) {
    if (unwantedAttributes.includes(node.attrs[i].name)) {
      node.attrs.splice(i, 1);
      i--;
    }
  }
  return node;
};

const tagShouldBeRemoved = (tagName: string) => {
  return ['h1', 'o:p', 'w:sdt'].includes(tagName);
};

const recurseNodes = (node, prefix) => {
  if (tagShouldBeRemoved(node.tagName)) {
    return;
  }
  if (node.tagName === 'img') {
    node = updateImageTag(node, prefix);
  } else if (node.tagName === 'td' || node.tagName === 'table') {
    node = removeUnwantedTableAttributes(node);
  } else if (node.tagName === 'a') {
    node = updateAnchorNameToId(node);
  } else {
    node = removeUnwantedAttributes(node);
  }

  for (let i = 0; i < node.childNodes?.length ?? 0; i++) {
    let returnedNode = recurseNodes(node.childNodes[i], prefix);
    if (returnedNode) {
      node.childNodes[i] = returnedNode;
    } else {
      node.childNodes.splice(i, 1);
      i--;
    }
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

  const { reportDirUrl, reportUrl } = await getReportUrl(reportName);
  console.log(`REPORT URL: ${reportUrl}`);
  const reportContent = await getReportHtmlContent(reportUrl);
  console.log(`REPORT CONTENT: ${reportContent}`);
  const htmlDocument = parse5.parse(reportContent, { scriptingEnabled: false });
  let htmlBody = getHtmlBody(htmlDocument);

  const assetsUrl = `${reportDirUrl}/assets/`;
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
