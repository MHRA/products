import TagManager from 'react-gtm-module';

const pushToDataLayer = (dataLayer: any) => {
  TagManager.dataLayer({
    dataLayer,
  });
};

interface ISearchEvent {
  searchTerm: string;
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
  viewPage: (pageName: string) => {
    pushToDataLayer({ event: pageName });
  },
};
