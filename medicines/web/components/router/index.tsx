import { useRouter } from 'next/router';
import { UrlObject } from 'url';
import { modifyUrl } from '../link';

export const useMhraRouter = () => {
  const nextRouter = useRouter();
  const push = nextRouter.push;
  nextRouter.push = (url: UrlObject) => push(modifyUrl(url));
  return nextRouter;
};
