import fs, { promises, readdirSync } from 'fs';
import moment from 'moment';
import path from 'path';
import { stringify } from 'querystring';
import { facetSearch } from '../services/azure-search';

const pagesDir = path.resolve('./dist');
const sitemapFile = path.resolve('./dist/sitemap.xml');
const robotsFile = path.resolve('./dist/robots.txt');

const BASE_URL = 'https://products.mhra.gov.uk';
const YYY_MM_DD = 'YYYY-MM-DD';
const CHANGE_FREQUENCY = 'daily';

const createFilePathsObj = async (): Promise<{ [index: string]: any }> => {
  const pages = readdirSync(pagesDir, {
    withFileTypes: true,
  })
    .filter(dirent => dirent.isDirectory())
    .map(dir => dir.name)
    .filter(dirs => !dirs.startsWith('_'))
    .map(dir => (dir === 'index' ? '/' : `/${dir}`));

  return pages.reduce(
    (acc, pageRoute) => ({
      ...acc,
      [`${pageRoute}`]: {
        page: pageRoute,
        lastModified: new Date().toISOString(),
      },
    }),
    {},
  );
};

const createSearchPathsObj = async (): Promise<{ [index: string]: any }> => {
  // Get substance and product page URLs by mimicing the behavour of the A–Z, 0–9 list
  // on the web site.
  const searchPathsObj: { [index: string]: any } = {};

  for (const letter of 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789') {
    const facetResult = await facetSearch(letter);

    for (const facet of facetResult[1].facets) {
      // Value is in the format "letter, substance, product". Substance and product may
      // not be present.
      const stringParts = facet.value.split(', ');
      const substance = stringParts[1];
      const product = stringParts[2];

      if (product) {
        // If a product is present, output a product URL.
        const route =
          '/?' +
          stringify({
            product: true,
            page: 1,
            search: product,
          });

        searchPathsObj[route] = {
          page: route,
          lastModified: new Date().toISOString(),
        };
      } else if (substance) {
        // If product is undefined and a substance is present, include a substance URL.
        const route =
          '/?' +
          stringify({
            substance,
          });

        searchPathsObj[route] = {
          page: route,
          lastModified: new Date().toISOString(),
        };
      }
    }
  }

  return searchPathsObj;
};

const createSiteMapString = async () => {
  const filePathsObj = await createFilePathsObj();
  const searchPathsObj = await createSearchPathsObj();
  const pathsObj = { ...filePathsObj, ...searchPathsObj };

  const urls = `${Object.keys(pathsObj)
    .map(
      path =>
        `<url>` +
        `<loc><![CDATA[${BASE_URL}${path}]]></loc>` +
        `<lastmod>${moment(pathsObj[path].lastModified).format(
          YYY_MM_DD,
        )}</lastmod>` +
        `<changefreq>${CHANGE_FREQUENCY}</changefreq>` +
        `</url>`,
    )
    .join('\n')}`;

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
  await fs.writeFileSync(sitemapFile, sitemapXml);
  await fs.writeFileSync(robotsFile, robotString);
};

if (!module.parent) {
  start().then(() =>
    process.stdout.write('Created sitemap.xml 🗺  & robots.txt 🤖\n'),
  );
}

export default start;
