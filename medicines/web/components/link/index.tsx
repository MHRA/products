import getConfig from 'next/config';
import Link, { LinkProps } from 'next/link';

class MhraLink extends Link {
  constructor(props: LinkProps) {
    const newProps: LinkProps = { ...props };

    if (typeof props.href === 'string') {
      newProps.href = getConfig().publicRuntimeConfig.baseUrl + props.href;
    } else {
      newProps.href = {
        pathname: getConfig().publicRuntimeConfig.baseUrl + props.href.pathname,
      };
    }
    super(newProps);
  }
}

export default MhraLink;
