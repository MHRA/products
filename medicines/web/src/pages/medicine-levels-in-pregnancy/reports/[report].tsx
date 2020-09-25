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
import {
  getHtmlBody,
  cleanUpHtml,
} from '../../../services/exported-html-sanitizer';

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

export const getStaticProps = async (context) => {
  const reportName = context.params.report;

  const { reportDirUrl, reportUrl } = await getReportUrl(reportName);
  console.log(`REPORT URL: ${reportUrl}`);
  const reportContent = await getReportHtmlContent(reportUrl);
  console.log(`REPORT CONTENT: ${reportContent}`);
  const htmlDocument = parse5.parse(reportContent, { scriptingEnabled: false });
  let htmlBody = getHtmlBody(htmlDocument);

  const assetsUrl = `${reportDirUrl}/assets/`;
  htmlBody = cleanUpHtml(htmlBody, assetsUrl);

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
