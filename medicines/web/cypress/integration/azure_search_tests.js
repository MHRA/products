// We need to replace window.fetch with an XHR-based implementation so that Cypress
// can mock out the endpoints we use.
// See https://docs.cypress.io/guides/guides/network-requests.html#Testing-Strategies
// and https://github.com/cypress-io/cypress/issues/95.

// Some code to do this is adapted from https://github.com/cypress-io/cypress-example-recipes/blob/master/examples/stubbing-spying__window-fetch/cypress/integration/polyfill-fetch-from-tests-spec.js

// UPDATE 27/07/20: we will no longer need to polyfill once full cypress mocking for fetch implemented when this PR is merged and released: https://github.com/cypress-io/cypress/pull/4176",
// Once this has been completed, it should be possible to upgrade next from 9.2 -> 9.4 - currently interaction causes fetch polyfill to break

let polyfill;

// grab fetch polyfill from remote URL, could be also from a local package
before(() => {
  const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js';

  cy.request(polyfillUrl).then((response) => {
    polyfill = response.body;
  });
});

Cypress.on('window:before:load', (win) => {
  delete win.fetch;
  // since the application code does not ship with a polyfill
  // load a polyfilled "fetch" from the test
  win.eval(polyfill);
  win.fetch = win.unfetch;

  // Clear out session storage so that the disclaimer is always presented.
  win.sessionStorage.clear();
});

const baseUrl = `https://${Cypress.env(
  'AZURE_SEARCH_SERVICE',
)}.search.windows.net/indexes/${Cypress.env('AZURE_SEARCH_INDEX')}/docs`;
const apiKey = `api-key=${Cypress.env(
  'AZURE_SEARCH_KEY',
)}&api-version=2017-11-11`;
const genericSearchParams = 'highlight=content&queryType=full&$count=true';

const mockParacetamolResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=(paracetamol~1+||+paracetamol^4)&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockParacetamolResultsPage2 = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=(paracetamol~1+||+paracetamol^4)&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockIbuprofenResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=(ibuprofen~1+||+ibuprofen^4)&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.json',
  );

const mockIbuprofenResultsPage2 = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=(ibuprofen~1+||+ibuprofen^4)&scoringProfile=preferKeywords&searchMode=all`,
    'fixture:search_results.page2.json',
  );

const mockIbuprofenSpcResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=(ibuprofen~1+||+ibuprofen^4)&scoringProfile=preferKeywords&searchMode=all&$filter=(doc_type+eq+'Spc')`,
    'fixture:search_results.spc.json',
  );

const mockIbuprofenSpcResultsPage2 = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=(ibuprofen~1+||+ibuprofen^4)&scoringProfile=preferKeywords&searchMode=all&$filter=(doc_type+eq+'Spc')`,
    'fixture:search_results.spc.page2.json',
  );

const mockIbuprofenSpcPilResults = () =>
  cy.route(
    `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=(ibuprofen~1+||+ibuprofen^4)&scoringProfile=preferKeywords&searchMode=all&$filter=(doc_type+eq+'Spc'+or+doc_type+eq+'Pil')`,
    'fixture:search_results.spcpil.json',
  );

const longerTimeout = 20000;

describe('Search', function () {
  it('can search for Paracetamol', function () {
    cy.server();
    mockParacetamolResults();
    mockParacetamolResultsPage2();
    cy.visit('/');
    cy.get("input[type='search']").type('paracetamol');
    cy.get('.searchbar').contains('Search').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });

  it('can filter for SPCs', function () {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.get('.searchbar').contains('Search').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Submit').click();
    cy.get("a[href='https://example.com/my-cool-document-spc.pdf']");
  });

  it('can filter for SPCs and PILs together', function () {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    mockIbuprofenSpcPilResults();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.get('.searchbar').contains('Search').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Patient Information Leaflet (PIL)').click();
    cy.contains('Submit').click();
    cy.get("a[href='https://example.com/my-cool-document-spc.pdf']");
    cy.get("a[href='https://example.com/my-cool-document-pil.pdf']");
  });

  it('can filter SPCs then go to next page to see 2nd page filtered documents', function () {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenSpcResults();
    mockIbuprofenSpcResultsPage2();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.get('.searchbar').contains('Search').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Submit').click();
    cy.get("a[href='https://example.com/an-example-par.pdf']").should(
      'not.exist',
    );
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document-spc-page2.pdf']");
    cy.get("a[href='https://example.com/dad-jokes-spc-page-2.pdf']");
  });

  it('can go to next page then filter SPCs to see 1st page filtered documents', function () {
    cy.server();
    mockIbuprofenResults();
    mockIbuprofenResults();
    mockIbuprofenResultsPage2();
    mockIbuprofenSpcResults();
    cy.visit('/');
    cy.get("input[type='search']").type('ibuprofen');
    cy.get('.searchbar').contains('Search').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/dad-jokes-page-2.pdf']");
    cy.contains('Summary of Product Characteristics (SPC)').click();
    cy.contains('Submit').click();
    cy.get("a[href='https://example.com/my-cool-document-spc.pdf']");
    cy.get("a[href='https://example.com/dad-jokes-spc.pdf']");
  });
});

describe('A-Z Index', function () {
  it('can navigate to Paracetamol via A-Z index', function () {
    cy.server();
    // Mock out list of substances.
    cy.route(
      `${baseUrl}?${apiKey}&facet=facets,count:50000,sort:value&$filter=facets/any(f:+f+eq+'P')&$top=0&searchMode=all`,
      'fixture:facets.json',
    );

    // Mock out first page of search results.
    cy.route(
      `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=0&search=&scoringProfile=preferKeywords&searchMode=all&$filter=product_name+eq+'PARACETAMOL+TABLETS'`,
      'fixture:search_results.json',
    );
    // Mock out second page of search results.
    cy.route(
      `${baseUrl}?${apiKey}&${genericSearchParams}&$top=10&$skip=10&search=&scoringProfile=preferKeywords&searchMode=all&$filter=product_name+eq+'PARACETAMOL+TABLETS`,
      'fixture:search_results.json',
    );

    cy.visit('/');
    cy.get('nav').contains('P').click();
    cy.contains('PARACETAMOL').click();
    cy.contains('PARACETAMOL TABLETS').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });
});

describe('Cookies', function () {
  const cookie_banner_text =
    'MHRA does not collect any data that would identify you directly. ' +
    'We would like to use Google Analytics to help us improve our services.';

  it("Cookies aren't accepted by default", function () {
    cy.visit('/');
    cy.contains(cookie_banner_text);
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'Off').find('input').should('be.checked');
    cy.contains('label', 'On').find('input').should('not.be.checked');
  });

  it('Accept cookies via the banner', function () {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains(cookie_banner_text).should('not.exist');
  });

  it('Accept cookies via the cookie policy form', function () {
    cy.visit('/');
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'On').click();
    cy.contains('Save your preferences').click();
    cy.contains(cookie_banner_text).should('not.exist');
  });

  it('Accepting cookies is reflected in cookie policy form', function () {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'On').find('input').should('be.checked');
    cy.contains('label', 'Off').find('input').should('not.be.checked');
  });

  it('Decline cookies via the cookie policy form', function () {
    cy.visit('/');
    cy.contains('Accept all cookies').click();
    cy.contains('Cookie Policy').click();
    cy.contains('label', 'Off').click();
    cy.contains('Save your preferences').click();
    cy.contains(cookie_banner_text);
  });
});
