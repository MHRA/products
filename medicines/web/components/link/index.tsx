import getConfig from 'next/config';
import Link, { LinkProps } from 'next/link';
import { UrlObject } from 'url';

type Url = string | UrlObject;
const MhraLink: React.FC<LinkProps> = props => {
  let href: Url;
  if (typeof props.href === 'string') {
    href = getConfig().publicRuntimeConfig.baseUrl + props.href;
  } else {
    href = {
      ...props.href,
      pathname: getConfig().publicRuntimeConfig.baseUrl + props.href.pathname,
    };
  }

  return <Link {...props} href={href} />;
};

export default MhraLink;
