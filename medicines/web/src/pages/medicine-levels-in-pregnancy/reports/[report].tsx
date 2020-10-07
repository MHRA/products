import React from 'react';
import styled from 'styled-components';

import { BmgfPage } from '../../../components/page';

import { useLocalStorage } from '../../../hooks';
import { mhraWhite, primaryColor, mhra70, mhra } from '../../../styles/colors';
import { getCleanedHtml } from '../../../services/exported-html-sanitizer';

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

  img {
    margin: 20px auto 30px;
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

    &:hover {
      background-color: ${mhra70};
      color: ${mhraWhite};
    }
  }
`;

const AccessibleHeading = styled.h2`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
`;

const ReportNotAvailable = () => (
  <div>Sorry - this report is currently unavailable.</div>
);

const Report = ({ reportName, htmlBody, pdfUrl }) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  return (
    <BmgfPage
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
          {htmlBody ? (
            <div
              dangerouslySetInnerHTML={{
                __html: htmlBody,
              }}
            ></div>
          ) : (
            <ReportNotAvailable />
          )}
        </section>
      </ReportBody>
    </BmgfPage>
  );
};

export default Report;

export const getStaticProps = async (context) => {
  const {
    getReportUrls,
    getReportHtmlContent,
  } = require('../../../services/bmgf-reports-fetcher');
  const fs = require('fs');

  const reportName = context.params.report;

  let htmlFilePaths = fs.readFileSync('./reports.json', 'utf8', (e) => {});
  htmlFilePaths = JSON.parse(htmlFilePaths);

  let pdfUrl;
  let assetsUrl;
  const htmlBody = await getReportUrls(reportName, htmlFilePaths)
    .then(({ reportPdfUrl, reportHtmlUrl, reportAssetsUrl }) => {
      pdfUrl = reportPdfUrl;
      assetsUrl = reportAssetsUrl;
      return getReportHtmlContent(reportHtmlUrl);
    })
    .then((reportContent) => getCleanedHtml(reportContent, assetsUrl))
    .catch(() => '');

  return {
    props: {
      htmlBody: htmlBody,
      reportName,
      pdfUrl,
    },
  };
};

export const getStaticPaths = async () => {
  const { getReportList } = require('../../../services/bmgf-reports-fetcher');
  const fs = require('fs');

  const htmlFilePaths = await getReportList();
  const staticPageNames = htmlFilePaths.map(
    (reportFilePath) =>
      `/medicine-levels-in-pregnancy/reports/${reportFilePath.split('/')[0]}`,
  );

  fs.writeFile('./reports.json', JSON.stringify(htmlFilePaths), (e) => {
    console.log(`Error writing report to disk: ${e}`);
  });

  return {
    paths: staticPageNames,
    fallback: false,
  };
};
