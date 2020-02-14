// We need to replace window.fetch with an XHR-based implementation so that Cypress
// can mock out the endpoints we use.
// See https://docs.cypress.io/guides/guides/network-requests.html#Testing-Strategies
// and https://github.com/cypress-io/cypress/issues/95.

// Some code to do this is adapted from https://github.com/cypress-io/cypress-example-recipes/blob/master/examples/stubbing-spying__window-fetch/cypress/integration/polyfill-fetch-from-tests-spec.js

let polyfill;
const baseUrl =
  'https://mhraproductsdev.search.windows.net/indexes/products-index/docs';
const apiKey =
  'api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11';
const genericSearchParams = 'highlight=content&queryType=full&$count=true';

// grab fetch polyfill from remote URL, could be also from a local package
before(() => {
  const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js';

  cy.request(polyfillUrl).then(response => {
    polyfill = response.body;
  });
});

Cypress.on('window:before:load', win => {
  delete win.fetch;
  // since the application code does not ship with a polyfill
  // load a polyfilled "fetch" from the test
  win.eval(polyfill);
  win.fetch = win.unfetch;

  // Clear out session storage so that the disclaimer is always presented.
  win.sessionStorage.clear();
});

const mockParacetamolResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockParacetamolResultsPage2 = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockIbuprofenResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=ibuprofen~1+ibuprofen^4&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockIbuprofenSpcResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=ibuprofen~1+ibuprofen^4&scoringProfile=preferKeywords&searchMode=all&$filter=doc_type+eq+'Spc'`,
    'fixture:search_results.spc.json',
  );

const mockIbuprofenSpcResultsPage2 = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=ibuprofen~1+ibuprofen^4&scoringProfile=preferKeywords&searchMode=all&$filter=doc_type+eq+'Spc'`,
    'fixture:search_results.spc.page2.json',
  );

const mockIbuprofenSpcPilResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=ibuprofen~1+ibuprofen^4&scoringProfile=preferKeywords&searchMode=all&$filter=doc_type+eq+'Spc'+or+doc_type+eq+'Pil'`,
    'fixture:search_results.spcpil.json',
  );

describe('Search', function() {
  it('can search for Paracetamol', function() {
    cy.server();
    mockParacetamolResults();
    mockParacetamolResultsPage2();
    cy.visit('/');
    cy.get("input[type='search']").type('paracetamol');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });

  it('can filter for SPCs', function() {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.get("a[href='https://example.com/my-cool-document-spc.pdf']");
  });

  it('can filter for SPCs and PILs together', function() {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    mockIbuprofenSpcPilResults();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Patient Information Leaflet (PIL)').click();
    cy.get("a[href='https://example.com/my-cool-document-spc.pdf']");
    cy.get("a[href='https://example.com/my-cool-document-pil.pdf']");
  });

  it('can filter SPCs then go to next page to see 2nd page filtered documents', function() {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    mockIbuprofenSpcResultsPage2();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document-spc-page2.pdf']");
    cy.get("a[href='https://example.com/dad-jokes-spc-page-2.pdf']");
  });

  it('can go to next page then filter SPCs to see 2nd page filtered documents', function() {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    mockIbuprofenSpcResultsPage2();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.get("a[href='https://example.com/my-cool-document-spc-page2.pdf']");
    cy.get("a[href='https://example.com/dad-jokes-spc-page-2.pdf']");
  });
});

describe('A-Z Index', function() {
  it('Navigate to Paracetamol via A-Z index', function() {
    cy.server();
    // Mock out list of substances and medcines.
    cy.route(
      `${baseUrl}?${apiKey}&facet=facets,count:50000,sort:value&$filter=facets/any(f:+f+eq+'P')&$top=0&searchMode=all`,
      'fixture:facets.json',
    );
    // Mock out first page of search results.
    cy.route(
      `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords&searchMode=all`,
      'fixture:search_results.json',
    );
    // Mock out second page of search results.
    cy.route(
      `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords&searchMode=all`,
      'fixture:search_results.json',
    );

    cy.visit('/');
    cy.get('nav')
      .contains('P')
      .click();
    cy.contains('PARACETAMOL').click();
    cy.contains('PARACETAMOL TABLETS').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });
});
