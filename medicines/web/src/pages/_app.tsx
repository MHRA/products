import { default as BaseApp } from 'next/app';
import Router from 'next/router';

import Events from '../services/events';

class App extends BaseApp {
  public componentDidMount(): void {
    Router.events.on('routeChangeComplete', Events.recordPageView);
  }
}

export default App;
