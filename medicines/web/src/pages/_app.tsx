import { default as BaseApp } from 'next/app';
import Router from 'next/router';
import ReactGA from 'react-ga-gtm';
import TagManager from 'react-gtm-module';

class App extends BaseApp {
  public componentDidMount(): void {
    TagManager.initialize({
      gtmId: process.env.GOOGLE_GTM_CONTAINER_ID as string,
    });

    ReactGA.initialize(process.env.GOOGLE_TRACKING_ID as string, {
      debug: true,
    });
    Router.events.on('routeChangeComplete', url => ReactGA.pageview(url));
  }
}

export default App;
