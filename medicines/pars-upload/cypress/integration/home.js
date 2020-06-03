import { setUp } from '../support/shared'

/* eslint-env node, mocha */

/* global Cypress, cy */

setUp()

describe('Home page', () => {
  it('can get to the upload new form page', () => {
    cy.server()

    cy.visit('/')

    cy.findByText('Upload a new document').click()

    cy.findByText('Continue').click()

    cy.findAllByText('New Public Assessment Report')
      .not('title')
      .should('have.length', 1)
  }),
    it('can get to the update PAR form page', () => {
      cy.server()

      cy.visit('/')

      cy.findByText('Update an existing document').click()

      cy.findByText('Continue').click()

      cy.findAllByText('Search for an existing report')
        .not('title')
        .should('have.length', 1)
    })
})