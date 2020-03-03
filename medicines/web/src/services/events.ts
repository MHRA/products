import TagManager from 'react-gtm-module';

const pushToDataLayer = (dataLayer: any) => {
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
