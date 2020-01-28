import { default as BaseApp } from 'next/app';
import Router from 'next/router';
import ReactGA from 'react-ga-gtm';

class App extends BaseApp {
  public componentDidMount(): void {
    Router.events.on('routeChangeComplete', url => ReactGA.pageview(url));
  }
}

export default App;
