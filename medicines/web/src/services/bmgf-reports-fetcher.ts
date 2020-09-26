const containerUrl = `https://${process.env.AZURE_STORAGE_ACCOUNT}.blob.core.windows.net/bmgf-docs`;

export const getReportList = async (): Promise<string[]> => {
  return fetch(`${containerUrl}/upload-index.txt`, {
    method: 'GET',
    headers: {
      'Content-Type': 'text/text',
    },
  })
    .then((response) => response.text())
    .then((text) => {
      return text.split('\n').filter((report) => report.length > 0);
    });
};

export const getReportUrls = async (reportName: string): Promise<any> => {
  return getReportList()
    .then((reportNamesAndFileNames) => {
      return reportNamesAndFileNames.find((reportNameAndFileName) =>
        reportNameAndFileName.startsWith(reportName),
      );
    })
    .then((reportNameAndFileName) => {
      return {
        reportPdfUrl: `${containerUrl}/${reportNameAndFileName}.pdf`,
        reportHtmlUrl: `${containerUrl}/${reportNameAndFileName}.pdf`,
        reportAssetsUrl: `${containerUrl}/${reportName}/assets/`,
      };
    });
};

export const getReportHtmlContent = async (
  reportUrl: string,
): Promise<string> => {
  return fetch(`${reportUrl}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'text/html',
    },
  }).then((response) => (response.ok ? response.text() : ''));
};
