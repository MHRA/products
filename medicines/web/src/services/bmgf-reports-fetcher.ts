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

export const getReportUrl = async (reportToGet: string): Promise<any> => {
  return getReportList()
    .then((reports) => {
      return reports.find((report) => report.startsWith(reportToGet));
    })
    .then((report) => `${containerUrl}/${report}`);
};

export const getReportHtmlContent = async (
  reportUrl: string,
): Promise<string> => {
  return fetch(`${reportUrl}.html`, {
    method: 'GET',
    headers: {
      'Content-Type': 'text/html',
    },
  }).then((response) => response.text());
};
