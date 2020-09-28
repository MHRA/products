import ReactGA from 'react-ga';
import TagManager from 'react-gtm-module';

let gaInitialized = false;
const useDebugScript = process.env.GOOGLE_USE_DEBUG === 'true';

const pushToDataLayer = (dataLayer: any) => {
  if (!gaInitialized) {
    return null;
  }

  dataLayer.date = getCurrentDateString();

  TagManager.dataLayer({
    dataLayer,
  });
  recordHistoryForNextEvent(dataLayer.event);
};

const getCurrentDateString = () => {
  const today = new Date();
  const dd = String(today.getDate()).padStart(2, '0');
  const mm = String(today.getMonth() + 1).padStart(2, '0');
  const yyyy = today.getFullYear();

  return yyyy + '-' + mm + '-' + dd;
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
    debug: useDebugScript,
  });
};

interface ISearchEvent {
  searchTerm: string;
  pageNo: number;
  docTypes?: string;
}

interface IProductSearchEvent {
  productName: string;
  pageNo: number;
  docTypes: string;
}

interface IPbpkSubstanceSearchEvent {
  substance: string;
  pageNo: number;
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
  viewPbpkResultsForSubstance: (substance: IPbpkSubstanceSearchEvent) => {
    pushToDataLayer({ event: 'substance-pbpk', ...substance });
  },
  viewPbpkSubstancesStartingWith: (letter: string) => {
    pushToDataLayer({ event: 'substance-index-pbpk', letter });
  },
  searchForPbpkReportsMatchingKeywords: (searchEvent: ISearchEvent) => {
    pushToDataLayer({
      event: 'search-pbpk',
      ...searchEvent,
    });
  },
  viewPage: (pageName: string) => {
    pushToDataLayer({ event: pageName });
  },
};
