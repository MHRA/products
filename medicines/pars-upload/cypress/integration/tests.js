describe('PARs upload homepage', function () {
  it('can get to the form page', function () {
    cy.server();

    cy.visit('/');

    cy.findByText('Upload a new document').click();

    cy.findByText('Continue').click();

    cy.findByText('New Public Assessment Report').should('exist');
  });
});
