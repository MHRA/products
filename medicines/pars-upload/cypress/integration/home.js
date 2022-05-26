import { setUp } from '../support/shared'

/* eslint-env node, mocha */

setUp()

describe('Home page', () => {
  it('can get to the upload new form page', () => {
    cy.visit('/')

    cy.findAllByText('What are you doing today?').should('exist')

    cy.findByText('Upload a new document').click()

    cy.findByText('Continue').click()

    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('New Public Assessment Report')
      })
  })
  it('can get to the update PAR form page', () => {
    cy.visit('/')

    cy.findAllByText('What are you doing today?').should('exist')

    cy.findByText('Update an existing document').click()

    cy.findByText('Continue').click()
    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('Search for an existing Public Assessment Report')
      })
  })
})
