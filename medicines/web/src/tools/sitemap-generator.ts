import fs, { readdirSync } from 'fs';
import moment from 'moment';
import path from 'path';
import { stringify } from 'querystring';
import {
  facetSearch,
  bmgfFacetSearch,
  IFacetResult,
} from '../services/azure-search';

const pagesDir = path.resolve('./dist');
const bmgfPagesDir = path.resolve(
  './dist/medicine-levels-in-pregnancy/reports',
);
const sitemapFile = path.resolve('./dist/sitemap.xml');
const robotsFile = path.resolve('./dist/robots.txt');

const BASE_URL = 'https://products.mhra.gov.uk';
const YYY_MM_DD = 'YYYY-MM-DD';
const CHANGE_FREQUENCY = 'daily';
const SHOW_BMGF = process.env.SHOW_BMGF;

const createFilePathsObj = async (
  dirName: string,
  pagePrefix: string,
): Promise<{ [index: string]: any }> => {
  const pages = readdirSync(dirName, {
    withFileTypes: true,
  })
    .filter((dirent) => dirent.isDirectory())
    .map((dir) => dir.name)
    .filter((dirs) => !dirs.startsWith('_'))
    .filter(
      (dirs) => SHOW_BMGF || !dirs.includes('medicine-levels-in-pregnancy'),
    )
    .map((dir) =>
      dir === 'index' ? `${pagePrefix}/` : `${pagePrefix}/${dir}`,
    );

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

const createSearchPathsObj = async (
  searchFacet: (letter: string) => Promise<[string, IFacetResult]>,
  routePrefix: string,
): Promise<{ [index: string]: any }> => {
  // Get substance and product page URLs by mimicing the behavour of the A–Z, 0–9 list
  // on the web site.
  const searchPathsObj: { [index: string]: any } = {};

  for (const letter of 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789') {
    const facetResult = await searchFacet(letter);

    for (const facet of facetResult[1].facets) {
      // Value is in the format "letter, substance, product". Substance and product may
      // not be present.
      const stringParts = facet.value.split(', ', 3);
      const index = stringParts[0];
      const substance = stringParts[1];
      const product = stringParts[2];

      if (product) {
        // If a product is present, output a product URL.
        const route =
          `${routePrefix}/product/?` +
          stringify({
            product: encodeURIComponent(product),
          });
        searchPathsObj[route] = {
          page: route,
          lastModified: new Date().toISOString(),
        };
      } else if (substance) {
        // If product is undefined and a substance is present, include a substance URL.
        const route =
          `${routePrefix}/substance/?` +
          stringify({
            substance: encodeURIComponent(substance),
          });
        searchPathsObj[route] = {
          page: route,
          lastModified: new Date().toISOString(),
        };
      } else if (index) {
        // If substance is undefined and an index is present, include an index URL.
        const route =
          `${routePrefix}/substance-index/?` +
          stringify({
            letter: index,
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
  const filePathsObj = await createFilePathsObj(pagesDir, '');
  const searchPathsObj = await createSearchPathsObj(facetSearch, '');
  let pathsObj = { ...filePathsObj, ...searchPathsObj };

  if (SHOW_BMGF) {
    const bmgfFilePathObj = await createFilePathsObj(
      bmgfPagesDir,
      '/medicine-levels-in-pregnancy/reports',
    );
    const bmgfSearchPathObj = await createSearchPathsObj(
      bmgfFacetSearch,
      '/medicine-levels-in-pregnancy',
    );
    pathsObj = { ...pathsObj, ...bmgfFilePathObj, ...bmgfSearchPathObj };
  }

  const urls = `${Object.keys(pathsObj)
    .map(
      (path) =>
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
