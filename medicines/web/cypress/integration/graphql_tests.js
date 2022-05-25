Cypress.on('window:before:load', (win) => {
  // Clear out session storage so that the disclaimer is always presented.
  win.sessionStorage.clear();
});

const graphQlUrl = Cypress.env('GRAPHQL_URL');

const mockParacetamolResultsForGraphQl = () =>
  cy.intercept('POST', graphQlUrl, { fixture: 'graphql-search-results' });

const longerTimeout = 20000;

describe('Search using GraphQl', function () {
  it('can search for Paracetamol', function () {
    mockParacetamolResultsForGraphQl();
    cy.visit('/search?search=paracetamol&page=1');
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });
});

describe('A-Z Index', function () {
  it('can navigate to Paracetamol Tablets with GraphQL feature on', function () {
    // Mock out GraphQL response.
    cy.intercept('POST', graphQlUrl, { fixture: 'graphql-substances-index' });

    cy.visit('/');
    cy.get('nav').contains('P').click();
    cy.contains('PARACETAMOL');

    cy.intercept('POST', graphQlUrl, { fixture: 'graphql-products-index' });

    cy.contains('PARACETAMOL').click();
    cy.contains('PARACETAMOL 500MG CAPSULES');

    cy.intercept('POST', graphQlUrl, { fixture: 'graphql-product-results' });

    cy.contains('PARACETAMOL 500MG CAPSULES').click();
    cy.contains('I have read and understand the disclaimer', {
      timeout: longerTimeout,
    }).click();
    cy.contains('Agree').click();
    cy.get("a[href='https://example.com/my-cool-document.pdf']");
  });
});

const mockParacetamolResultsForMedicineLevelsInPregnancyGraphQl = () =>
  cy.intercept('POST', graphQlUrl, { fixture: 'graphql-search-results-bmgf' });

describe('Search medicine levels in pregnancy docs using GraphQl', function () {
  it('can search for Paracetamol', function () {
    mockParacetamolResultsForMedicineLevelsInPregnancyGraphQl();
    cy.visit('/medicine-levels-in-pregnancy/search?search=paracetamol&page=1');
    cy.get(
      "a[href='/medicine-levels-in-pregnancy/reports/Example report name2']",
    );
  });
});

describe('A-Z Index for medicine levels in pregnancy', function () {
  it('can navigate to Paracetamol Tablets with GraphQL feature on', function () {
    // Mock out GraphQL response.
    cy.intercept('POST', graphQlUrl, {
      fixture: 'graphql-substances-index-bmgf',
    });

    cy.visit('/medicine-levels-in-pregnancy');
    cy.get('nav').contains('P').click();
    cy.contains('PARACETAMOL');

    cy.intercept('POST', graphQlUrl, {
      fixture: 'graphql-substance-results-bmgf',
    });

    cy.contains('PARACETAMOL').click();

    cy.get(
      "a[href='/medicine-levels-in-pregnancy/reports/Example report name2']",
    );
  });
});
