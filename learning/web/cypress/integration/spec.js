describe('Navigate website', function() {
  it('Can navigate through website', function() {
    cy.visit('/');
    cy.contains("Antipsychotics").click();
    cy.contains("Extrapyramidal side effects: acute dystonia").click()
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
