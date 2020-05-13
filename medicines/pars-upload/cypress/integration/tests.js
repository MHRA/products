describe('Home page', () => {
  it('can get to the form page', () => {
    cy.server();

    cy.visit('/');

    cy.findByText('Upload a new document').click();

    cy.findByText('Continue').click();

    cy.findByText('New Public Assessment Report').should('exist');
  });
});

describe('PARs upload form', () => {
  it('can add multiple substances', () => {
    cy.visit('/new-par');

    cy.findByLabelText('Product name').type('Ibuprofen pills');

    cy.findByLabelText('Strength').type('Really powerful stuff');

    cy.findByLabelText('Pharmaceutical dose form').type('some form');

    cy.findByLabelText('Active substance').type('Ibuprofen');

    cy.findByText('Add another active substance').click();

    cy.findAllByLabelText('Active substance').last().type('Paracetamol');

    cy.findByText('Add another active substance').click();

    cy.findAllByLabelText('Active substance').last().type('Temazepam');

    cy.findAllByLabelText('Active substance').should('have.length', 3);
  });

  it('can add multiple products', () => {
    const productName = 'Ibuprofen pills';

    cy.visit('/new-par');

    cy.findByLabelText('Product name').type(productName);

    cy.findByLabelText('Strength').type('Really powerful stuff');

    cy.findByLabelText('Pharmaceutical dose form').type('some form');

    cy.findByLabelText('Active substance').type('Ibuprofen');

    cy.findByText('Add another active substance').click();

    cy.findAllByLabelText('Active substance').last().type('Paracetamol');

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select('HR');
        cy.findByLabelText('First chunk').type('12345');
        cy.findByLabelText('Second chunk').type('ABC123');
      });

    cy.findByText('Add another product').click();

    cy.findByText(productName).should('exist');
  });
});
