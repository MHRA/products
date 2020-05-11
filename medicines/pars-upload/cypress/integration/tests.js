describe('PARs upload homepage', function () {
  it('can get to the form page', function () {
    cy.server();
    cy.visit('/');
    cy.contains('Upload a new document').click();
    cy.get("input[type='text']").type('Craig');
  });
});
