export const getReportList = async (
  containerUrl: string,
): Promise<string[]> => {
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

const getReportUrl = async (
  reportToGet: string,
  containerUrl: string,
): Promise<any> => {
  return getReportList(containerUrl)
    .then((reports) => {
      return reports.find((report) => report.startsWith(reportToGet));
    })
    .then((report) => `${containerUrl}/${report}`);
};

export const getReportContent = async (
  report: string,
  containerUrl: string,
): Promise<string> => {
  return getReportUrl(report, containerUrl).then((reportUrl) => {
    return fetch(reportUrl, {
      method: 'GET',
      headers: {
        'Content-Type': 'text/html',
      },
    }).then((response) => response.text());
  });
};
