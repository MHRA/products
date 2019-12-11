const fs = require('fs').promises;
const path = require('path');

const pagesDir = path.resolve('./pages');
const sitemapFile = path.resolve('./dist/sitemap.xml');

const BASE_URL = 'https://products.gov.uk';

const createPathsObj = async () => {
  const allFiles = await fs.readdir(pagesDir);
  const pages = allFiles
    .filter(file => !file.startsWith('_'))
    .map(file => file.slice(0, -4))
    .map(file => (file === 'index' ? '/' : `/${file}`));

  return pages.reduce(
    (acc, pageRoute) =>
      Object.assign(acc, {
        [`${pageRoute}`]: {
          page: pageRoute,
          lastModified: new Date().toISOString(),
        },
      }),
    {},
  );
};

const formatDate = date => {
  var d = new Date(date),
    month = '' + (d.getMonth() + 1),
    day = '' + d.getDate(),
    year = d.getFullYear();

  if (month.length < 2) month = '0' + month;
  if (day.length < 2) day = '0' + day;

  return [year, month, day].join('-');
};

const createSiteMapString = async () => {
  const pathsObj = await createPathsObj();

  console.log({ pathsObj });

  const sitemapXml = `<?xml version="1.0" encoding="UTF-8"?>
  <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"> 
    ${Object.keys(pathsObj)
      .map(
        path => `<url>
      <loc>${BASE_URL}${path}</loc>
      <lastmod>${formatDate(new Date(pathsObj[path].lastModified))}</lastmod>
    </url>`,
      )
      .join('\n    ')}
  </urlset>`;

  return sitemapXml;
};

const start = async () => {
  const sitemapXml = await createSiteMapString();
  fs.writeFile(sitemapFile, sitemapXml);
};

start();
