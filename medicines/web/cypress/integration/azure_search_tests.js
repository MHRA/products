Cypress.on('window:before:load', (win) => {
  // Clear out session storage so that the disclaimer is always presented.
  win.sessionStorage.clear();
});

const baseUrl = `https://${Cypress.env(
  'AZURE_SEARCH_SERVICE',
)}.search.windows.net/indexes/${Cypress.env('AZURE_SEARCH_INDEX')}/docs`;
const apiKey = `api-key=${Cypress.env(
  'AZURE_SEARCH_KEY',
)}&api-version=2017-11-11`;
const genericSearchParams = 'highlight=content&queryType=full&%24count=true';

const mockParacetamolResults = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=%28paracetamol%7E1+%7C%7C+paracetamol%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results' },
  );

const mockParacetamolResultsPage2 = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=%28paracetamol%7E1+%7C%7C+paracetamol%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results' },
  );

const mockIbuprofenResults = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=%28ibuprofen%7E1+%7C%7C+ibuprofen%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results' },
  );

const mockIbuprofenResultsPage2 = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=%28ibuprofen%7E1+%7C%7C+ibuprofen%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results.page2' },
  );

const mockIbuprofenSpcResults = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=%28ibuprofen%7E1+%7C%7C+ibuprofen%5E4%29&scoringProfile=preferKeywords&searchMode=all&%24filter=%28doc_type+eq+%27Spc%27%29`,
    { fixture: 'search_results.spc' },
  );

const mockIbuprofenSpcResultsPage2 = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=%28ibuprofen%7E1+%7C%7C+ibuprofen%5E4%29&scoringProfile=preferKeywords&searchMode=all&%24filter=%28doc_type+eq+%27Spc%27%29`,
    { fixture: 'search_results.spc.page2' },
  );

const mockIbuprofenSpcPilResults = () =>
  cy.intercept(
    `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=%28ibuprofen%7E1+%7C%7C+ibuprofen%5E4%29&scoringProfile=preferKeywords&searchMode=all&%24filter=%28doc_type+eq+%27Spc%27+or+doc_type+eq+%27Pil%27%29`,
    { fixture: 'search_results.spcpil' },
  );

const longerTimeout = 20000;

describe('Search', function () {
  it('can search for Paracetamol', function () {
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
    // Mock out list of substances.
    cy.intercept(
      `${baseUrl}?${apiKey}&facet=facets%2Ccount%3A50000%2Csort%3Avalue&%24filter=facets%2Fany%28f%3A+f+eq+%27P%27%29&%24top=0&searchMode=all`,
      { fixture: 'facets' },
    );

    // Mock out first page of search results.
    cy.intercept(
      `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=&scoringProfile=preferKeywords&searchMode=all&%24filter=product_name+eq+%27PARACETAMOL+TABLETS%27`,
      { fixture: 'search_results' },
    );
    // Mock out second page of search results.
    cy.intercept(
      `${baseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=&scoringProfile=preferKeywords&searchMode=all&%24filter=product_name+eq+%27PARACETAMOL+TABLETS%27`,
      { fixture: 'search_results' },
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

const bmgfBaseUrl = `https://${Cypress.env(
  'AZURE_SEARCH_SERVICE',
)}.search.windows.net/indexes/${Cypress.env('BMGF_AZURE_SEARCH_INDEX')}/docs`;
const mockBmgfParacetamolResults = () =>
  cy.intercept(
    `${bmgfBaseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=%28paracetamol%7E1+%7C%7C+paracetamol%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results_bmgf' },
  );

const mockBmgfParacetamolResultsPage2 = () =>
  cy.intercept(
    `${bmgfBaseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=%28paracetamol%7E1+%7C%7C+paracetamol%5E4%29&scoringProfile=preferKeywords&searchMode=all`,
    { fixture: 'search_results_bmgf.page2' },
  );

describe('Search for medicine levels in pregnancy', function () {
  it('can search for Paracetamol', function () {
    mockBmgfParacetamolResults();
    mockBmgfParacetamolResultsPage2();
    cy.visit('/medicine-levels-in-pregnancy');
    cy.get("input[type='search']").type('paracetamol');
    cy.get('.searchbar').contains('Search').click();
    cy.get("a[href='/medicine-levels-in-pregnancy/reports/Example report 1']");
    cy.contains('Next').click();
    cy.get("a[href='/medicine-levels-in-pregnancy/reports/Example report 4']");
  });
});

describe('A-Z Index for medicine levels in pregnancy', function () {
  it('can navigate to Paracetamol via A-Z index', function () {
    // Mock out list of substances.
    cy.intercept(
      `${bmgfBaseUrl}?${apiKey}&facet=facets%2Ccount%3A50000%2Csort%3Avalue&%24filter=facets%2Fany%28f%3A+f+eq+%27P%27%29&%24top=0&searchMode=all`,
      { fixture: 'facets' },
    );

    // Mock out first page of search results.
    cy.intercept(
      `${bmgfBaseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=0&search=&scoringProfile=preferKeywords&searchMode=all&%24filter=active_substances%2Fany%28substance%3A+substance+eq+%27PARACETAMOL%27%29`,
      { fixture: 'search_results_bmgf' },
    );

    // Mock out second page of search results.
    cy.intercept(
      `${bmgfBaseUrl}?${apiKey}&${genericSearchParams}&%24top=10&%24skip=10&search=&scoringProfile=preferKeywords&searchMode=all&%24filter=active_substances%2Fany%28substance%3A+substance+eq+%27PARACETAMOL%27%29`,
      { fixture: 'search_results_bmgf.page2' },
    );

    cy.visit('/medicine-levels-in-pregnancy');
    cy.get('nav').contains('P').click();
    cy.contains('PARACETAMOL').click();
    cy.get("a[href='/medicine-levels-in-pregnancy/reports/Example report 1']");
    cy.contains('Next').click();
    cy.get("a[href='/medicine-levels-in-pregnancy/reports/Example report 4']");
  });
});
