import getConfig from 'next/config';
import Link, { LinkProps } from 'next/link';
import { UrlObject } from 'url';

const deduplicateSlashes = (url: string) => {
  return url.replace(/\/+/g, '/');
};

type Url = string | UrlObject;
const MhraLink: React.FC<LinkProps> = props => {
  const { baseUrl, appendix } = getConfig().publicRuntimeConfig;
  let href: Url;
  if (typeof props.href === 'string') {
    href = deduplicateSlashes(`${baseUrl}${props.href}${appendix}`);
  } else {
    href = {
      ...props.href,
      pathname: deduplicateSlashes(
        `${baseUrl}${props.href.pathname}${appendix}`,
      ),
    };
  }

  return <Link {...props} href={href} />;
};

export default MhraLink;
