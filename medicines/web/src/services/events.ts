import ReactGA from 'react-ga';
import TagManager from 'react-gtm-module';

let gaInitialized = false;

const pushToDataLayer = (dataLayer: any) => {
  if (!gaInitialized) {
    return null;
  }

  TagManager.dataLayer({
    dataLayer,
  });
  recordHistoryForNextEvent(dataLayer.event);
};

const recordHistoryForNextEvent = (event: string) => {
  TagManager.dataLayer({
    dataLayer: {
      previousEvent: event,
      pageCategory: event,
    },
  });
};

const recordPageView = (url: string) => {
  if (!gaInitialized) {
    return null;
  }
  ReactGA.pageview(url);
};

const initializeTrackingScripts = () => {
  if (gaInitialized) {
    return null;
  }
  gaInitialized = true;
  TagManager.initialize({
    gtmId: process.env.GOOGLE_GTM_CONTAINER_ID as string,
    dataLayerName: 'dataLayer',
  });
  ReactGA.initialize(process.env.GOOGLE_TRACKING_ID as string, {
    debug: true,
  });
};

interface ISearchEvent {
  searchTerm: string;
  pageNo: number;
  docTypes: string;
}

interface IProductSearchEvent {
  productName: string;
  pageNo: number;
  docTypes: string;
}

export default {
  initializeTrackingScripts,
  recordPageView,
  searchForProductsMatchingKeywords: (searchEvent: ISearchEvent) => {
    pushToDataLayer({
      event: 'search',
      ...searchEvent,
    });
  },
  viewProductsForSubstance: (substance: string) => {
    pushToDataLayer({ event: 'substance', substance });
  },
  viewSubstancesStartingWith: (letter: string) => {
    pushToDataLayer({ event: 'drugIndex', letter });
  },
  viewResultsForProduct: (productSearch: IProductSearchEvent) => {
    pushToDataLayer({ event: 'product', ...productSearch });
  },
  viewPage: (pageName: string) => {
    pushToDataLayer({ event: pageName });
  },
};
