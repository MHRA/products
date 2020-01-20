describe('Search', function() {
  let polyfill

  // grab fetch polyfill from remote URL, could be also from a local package
  before(() => {
    const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js'

    cy.request(polyfillUrl)
    .then((response) => {
      polyfill = response.body
    })
  })

  it('Search for Paracetamol', function() {
    cy.server();
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=0&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords',
      "fixture:search_results.json");
    cy.route(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=10&search=paracetamol~1+paracetamol^4&scoringProfile=preferKeywords',
      "fixture:search_results.json");
    cy.visit('/', {
      onBeforeLoad: win => {
        delete win.fetch;
        // since the application code does not ship with a polyfill
        // load a polyfilled "fetch" from the test
        win.eval(polyfill);
        win.fetch = win.unfetch;
        win.sessionStorage.clear();
      },
    });
    cy.get('input[type=\'search\']').type('paracetamol');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get('a[href=\'https://example.com/my-cool-document.pdf\']');
  });
});
describe('A-Z Index', function() {
  let polyfill

  // grab fetch polyfill from remote URL, could be also from a local package
  before(() => {
    const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js'

    cy.request(polyfillUrl)
    .then((response) => {
      polyfill = response.body
    })
  })
  it('Navigate to Paracetamol via A-Z index', function() {
    cy.server();
    cy.route(
      "https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&facet=facets,count:50000,sort:value&$filter=facets/any(f:+f+eq+'P')&$top=0",
      "fixture:facets.json");
    cy.route(
      "https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=0&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords",
      "fixture:search_results.json");
    cy.route(
      "https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&$count=true&$top=10&$skip=10&search=PARACETAMOL~1+PARACETAMOL^4+TABLETS~1+TABLETS^4&scoringProfile=preferKeywords",
      "fixture:search_results.json");
    
    cy.visit('/', {
      onBeforeLoad: win => {
        delete win.fetch;
        // since the application code does not ship with a polyfill
        // load a polyfilled "fetch" from the test
        win.eval(polyfill);
        win.fetch = win.unfetch;
        win.sessionStorage.clear();
      },
    });
    cy.get('nav')
      .contains('P')
      .click();
    cy.contains('PARACETAMOL').click();
    cy.contains('PARACETAMOL TABLETS').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get('a[href=\'https://example.com/my-cool-document.pdf\']');
  });
});
