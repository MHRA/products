import App from 'next/app';
import Router from 'next/router';
import ReactGA from 'react-ga-gtm';

ReactGA.initialize(process.env.GOOGLE_TRACKING_ID as string);
Router.events.on('routeChangeComplete', url => ReactGA.pageview(url));

export default App;
