describe('XHR test', function() {
  it('My XHR test runs OK.', function() {
    cy.visit('https://www.craiga.id.au/xhr');
    cy.contains('This page made an XML HTTP request');
  });
});
describe('Search', function() {
  it('Can at least get to Azure search thingy', function() {
    cy.request(
      'https://mhraproductsdev.search.windows.net/indexes/products-index/docs?api-key=CFBCBE8AA11AA871C14001527533870C&api-version=2017-11-11&highlight=content&queryType=full&%24count=true&%24top=10&%24skip=0&search=paracetamol%7E1+paracetamol%5E4&scoringProfile=preferKeywords',
    );
  });
  it('Search for Paracetamol', function() {
    cy.visit('/', {
      onBeforeLoad: win => {
        win.sessionStorage.clear();
      },
    });
    cy.get("input[type='search']").type('paracetamol');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href*='docs']");
  });
  it('Directly visit a search for Paracetamol', function() {
    cy.visit('/?search=paracetamol&page=1', {
      onBeforeLoad: win => {
        win.sessionStorage.clear();
      },
    });
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href*='docs']");
  });
  it('Search for Paracetamol on production', function() {
    cy.visit('https://products.mhra.gov.uk/', {
      onBeforeLoad: win => {
        win.sessionStorage.clear();
      },
    });
    cy.get("input[type='search']").type('paracetamol');
    cy.contains('Search').click();
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href*='docs']");
  });
});
describe('A-Z Index', function() {
  it('Navigate to Paracetamol via A-Z index', function() {
    cy.visit('/', {
      onBeforeLoad: win => {
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
    cy.get("a[href*='docs']");
  });
});
