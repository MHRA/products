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

export default {
  searchForProductsMatchingKeywords: (searchTerm: string, pageNo: number) => {
    pushToDataLayer({
      event: 'search',
      searchTerm,
      pageNo,
    });
  },
  viewProductsForSubstance: (substance: string) => {
    pushToDataLayer({ event: 'substance', substance });
  },
  viewSubstancesStartingWith: (letter: string) => {
    pushToDataLayer({ event: 'drugIndex', letter });
  },
  viewPage: (pageName: string) => {
    pushToDataLayer({ event: pageName });
  },
};
