const fs = require('fs').promises;
const path = require('path');
const moment = require('moment');

const pagesDir = path.resolve('./pages');
const sitemapFile = path.resolve('./dist/sitemap.xml');
const robotsFile = path.resolve('./dist/robots.txt');

const BASE_URL = 'https://products.gov.uk';
const YYY_MM_DD = 'YYYY-MM-DD';
const CHANGE_FREQUENCY = 'daily';

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

const createSiteMapString = async () => {
  const pathsObj = await createPathsObj();

  const urls = `${Object.keys(pathsObj)
    .map(
      path =>
        `<url>
          <loc>${BASE_URL}${path}</loc>
          <lastmod>${moment(pathsObj[path].lastModified).format(
            YYY_MM_DD,
          )}</lastmod>
          <changefreq>${CHANGE_FREQUENCY}</changefreq>
        </url>
        `,
    )
    .join('')}`;

  const sitemapXml = `<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"> 
      ${urls}
    </urlset>`;

  return sitemapXml;
};

const robotString = `
User-agent: *

Sitemap: ${BASE_URL}/sitemap.xml
`;

const start = async () => {
  const sitemapXml = await createSiteMapString();
  await fs.writeFile(sitemapFile, sitemapXml);
  await fs.writeFile(robotsFile, robotString);
};

if (!module.parent) {
  start().then(() => console.log('Created sitemap.xml 🗺  & robots.txt 🤖'));
}

module.exports = start;
