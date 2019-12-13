import getConfig from 'next/config';
import Link, { LinkProps } from 'next/link';
import { UrlObject } from 'url';

type Url = string | UrlObject;
const MhraLink: React.FC<LinkProps> = props => {
  const { baseUrl, appendix } = getConfig().publicRuntimeConfig;
  let href: Url;
  if (typeof props.href === 'string') {
    href = `${baseUrl}${props.href}${appendix}`;
  } else {
    href = {
      ...props.href,
      pathname: `${baseUrl}${props.href.pathname}${appendix}`,
    };
  }

  return <Link {...props} href={href} />;
};

export default MhraLink;
