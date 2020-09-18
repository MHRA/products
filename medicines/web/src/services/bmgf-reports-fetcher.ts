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

// const response = await fetch(
//   `https://mhraproductsnonprod.blob.core.windows.net/bmgf-docs/example_report_references_exported_as_html.html`,
// );
// if (!response.ok) {
//   return;
// }
// let body = await response.text();

// export const getMarkdownDoc = async (url: string): Promise<any> => {
//   const resp: Response = await fetch(url, {
//     method: 'GET',
//     // headers: {
//     //   'Content-Type': 'text/markdown',
//     // },
//   });

//   if (resp.ok) {
//     return resp;
//   }
// };
