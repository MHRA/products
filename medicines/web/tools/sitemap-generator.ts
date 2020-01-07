import fs, { promises, readdirSync } from 'fs';
import moment from 'moment';
import path from 'path';

const pagesDir = path.resolve('./dist');
const sitemapFile = path.resolve('./dist/sitemap.xml');
const robotsFile = path.resolve('./dist/robots.txt');

const BASE_URL = 'https://products.mhra.gov.uk';
const YYY_MM_DD = 'YYYY-MM-DD';
const CHANGE_FREQUENCY = 'daily';

const createPathsObj = async (): Promise<{ [index: string]: any }> => {
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
  await fs.writeFileSync(sitemapFile, sitemapXml);
  await fs.writeFileSync(robotsFile, robotString);
};

if (!module.parent) {
  start().then(() =>
    process.stdout.write('Created sitemap.xml 🗺  & robots.txt 🤖\n'),
  );
}

export default start;
