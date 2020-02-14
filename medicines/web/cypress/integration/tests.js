// We need to replace window.fetch with an XHR-based implementation so that Cypress
// can mock out the endpoints we use.
// See https://docs.cypress.io/guides/guides/network-requests.html#Testing-Strategies
// and https://github.com/cypress-io/cypress/issues/95.

// Some code to do this is adapted from https://github.com/cypress-io/cypress-example-recipes/blob/master/examples/stubbing-spying__window-fetch/cypress/integration/polyfill-fetch-from-tests-spec.js

let polyfill;

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

describe('Search', function() {
  it('Search for Paracetamol', function() {
    cy.server();
    // Mock out first page of search results.
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=0&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords&searchMode=all',
      'fixture:search_results.json',
    );
    // Mock out second page of search results.
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=10&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords&searchMode=all',
      'fixture:search_results.json',
    );

    cy.visit('/');
    cy.get("input[type='search']").type('paracetamol');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });
});

describe('A-Z Index', function() {
  it('Navigate to Paracetamol via A-Z index', function() {
    cy.server();
    // Mock out list of substances and medcines.
    cy.route(
      "https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&facet=facets,count:50000,sort:value&$filter=facets/any(f:+f+eq+'P')&$top=0&searchMode=all",
      'fixture:facets.json',
    );
    // Mock out first page of search results.
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=0&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords&searchMode=all',
      'fixture:search_results.json',
    );
    // Mock out second page of search results.
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=10&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords&searchMode=all',
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

describe('Cookies', function() {
  const cookie_banner_text =
    'MHRA does not collect any data that would identify you directly. ' +
    'We would like to use Google Analytics to help us improve our services.';

  it("Cookies aren't accepted by default", function() {
    cy.visit('/');
    cy.contains(cookie_banner_text);
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'Off')
      .find('input')
      .should('be.checked');
    cy.contains('label', 'On')
      .find('input')
      .should('not.be.checked');
  });

  it('Accept cookies via the banner', function() {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains(cookie_banner_text).should('not.exist');
  });

  it('Accept cookies via the cookie policy form', function() {
    cy.visit('/');
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'On').click();
    cy.contains('Save your preferences').click();
    cy.contains(cookie_banner_text).should('not.exist');
  });

  it('Accepting cookies is reflected in cookie policy form', function() {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'On')
      .find('input')
      .should('be.checked');
    cy.contains('label', 'Off')
      .find('input')
      .should('not.be.checked');
  });

  it('Decline cookies via the cookie policy form', function() {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'Off').click();
    cy.contains('Save your preferences').click();
    cy.contains(cookie_banner_text);
  });
});
