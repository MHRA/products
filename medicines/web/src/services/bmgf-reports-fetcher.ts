import {
  BlobServiceClient,
  StorageSharedKeyCredential,
} from '@azure/storage-blob';

const containerUrl = `https://${process.env.AZURE_STORAGE_ACCOUNT}.blob.core.windows.net/bmgf-docs`;
const account = process.env.AZURE_STORAGE_ACCOUNT || '';
const accountKey = process.env.AZURE_STORAGE_KEY || '';

export const getReportList = async (): Promise<string[]> => {
  const sharedKeyCredential = new StorageSharedKeyCredential(
    account,
    accountKey,
  );
  const blobServiceClient = new BlobServiceClient(
    `https://${account}.blob.core.windows.net`,
    sharedKeyCredential,
  );

  const containerName = 'bmgf-docs';
  const containerClient = blobServiceClient.getContainerClient(containerName);

  const blobs = containerClient.listBlobsFlat();
  const reportNames = [];
  for await (const blob of blobs) {
    if (blob.name.endsWith('.html')) {
      reportNames.push(blob.name);
    }
  }
  return reportNames;
};

export const getReportUrls = async (
  reportName: string,
  fileNames: string[],
): Promise<any> => {
  const fileName = fileNames.find((fileName) =>
    fileName.startsWith(reportName),
  );
  return {
    reportPdfUrl: `${containerUrl}/${fileName?.replace('.html', '.pdf')}`,
    reportHtmlUrl: `${containerUrl}/${fileName}`,
    reportAssetsUrl: `${containerUrl}/${reportName}/assets/`,
  };
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
