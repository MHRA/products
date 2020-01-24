describe('Navigate website', function() {
  it('Can navigate through website', function() {
    cy.visit('/');
    cy.contains("Antipsychotics").click();
    cy.contains("Extrapyramidal side effects: acute dystonia").click()
  });
});
