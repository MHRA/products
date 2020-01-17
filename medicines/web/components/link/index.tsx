import getConfig from 'next/config';
import Link, { LinkProps } from 'next/link';
import { UrlObject } from 'url';

const deduplicateSlashes = (url: string) => {
  return url.replace(/\/+/g, '/');
};

type Url = string | UrlObject;
const MhraLink: React.FC<LinkProps> = props => {
  const href: Url = modifyUrl(props.href);

  return <Link {...props} href={href} />;
};

export default MhraLink;

export function modifyUrl(url: Url) {
  const { baseUrl, appendix } = getConfig().publicRuntimeConfig;
  let href: Url;
  if (typeof url === 'string') {
    href = deduplicateSlashes(`${baseUrl}${url}${appendix}`);
  } else {
    href = {
      ...url,
      pathname: deduplicateSlashes(`${baseUrl}${url.pathname}${appendix}`),
    };
  }
  return href;
}
