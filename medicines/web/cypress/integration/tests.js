describe('Search', function() {
  it('Search for Paracetamol', function() {
    cy.visit('/?search=paracetamol&page=1', {
      onBeforeLoad: win => {
        win.sessionStorage.clear();
      },
    });
    cy.contains('I have read and understand the disclaimer').click();
    cy.contains('Agree').click();
    cy.contains('Next').click();
    cy.get("a[href*='docs/']");
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
    cy.get("a[href*='docs/']");
  });
});
