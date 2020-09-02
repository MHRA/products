import {
  addAndDeleteSubstances,
  addAndDeleteProducts,
  addDuplicateLicenceNumbers,
  completeUploadForm,
  completeUploadFile,
  mockSuccessfulSubmission,
  setUp,
} from '../support/shared'

/* eslint-env node, mocha */

/* global Cypress, cy */

setUp()

const parsUrl = Cypress.env('PARS_UPLOAD_URL')
const baseUrl = Cypress.config().baseUrl

const startNewParForm = () => {
  cy.visit('/')

  cy.findAllByText('What are you doing today?').should('exist')

  cy.findByText('Upload a new document').click()

  cy.findByText('Continue').click()
}

describe('New PARs upload', () => {
  it('can add and delete multiple substances', () => {
    startNewParForm()

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substance1: 'Ibuprofen',
      substance2: 'Paracetamol',
      substance3: 'Temazepam',
    }
    let uploadPageTitle = 'New Public Assessment Report'
    addAndDeleteSubstances(uploadData, uploadPageTitle)
  })

  it('can add and delete multiple products', () => {
    startNewParForm()

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substance1: 'Ibuprofen',
      substance2: 'Paracetamol',
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    addAndDeleteProducts(uploadData, uploadPageTitle)
  })
  it('duplicate licence numbers are not allowed', () => {
    startNewParForm()

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: 'Ibuprofen',
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    addDuplicateLicenceNumbers(uploadData, uploadPageTitle)
  })
  it('upload field only accepts PDFs', () => {
    startNewParForm()

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    completeUploadForm(uploadData, uploadPageTitle)

    const fileName = 'rabbit-anti-human-stuff.txt'
    const expectedTitle = 'Upload your PDF'
    completeUploadFile(fileName, expectedTitle, 'text/plain', 'ascii')

    cy.once('fail', (err) => {
      expect(err.message).to.include(
        'One or more field is invalid within given file(s)'
      )
    })
  })
  it('review page shows the correct information', () => {
    startNewParForm()
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    completeUploadForm(uploadData, uploadPageTitle)

    const fileName = 'rabbit-anti-human-stuff.pdf'
    const expectedTitle = 'Upload your PDF'
    completeUploadFile(fileName, expectedTitle)

    cy.findAllByText('Check your answers before sending the report')
      .not('title')
      .should('have.length', 1)

    cy.findByText('Brand/Generic name')
      .parent()
      .within(() => {
        cy.findByText(uploadData.brand).should('exist')
      })

    cy.findByText('Strength')
      .parent()
      .within(() => {
        cy.findByText(uploadData.strength).should('exist')
      })

    cy.findByText('Pharmaceutical dose form')
      .parent()
      .within(() => {
        cy.findByText(uploadData.doseForm).should('exist')
      })

    cy.findByText('Active substances')
      .parent()
      .within(() => {
        cy.findByText(uploadData.substances.join(', ')).should('exist')
      })

    cy.findByText('Licence number')
      .parent()
      .within(() => {
        cy.findByText(
          `${uploadData.licence.type} ${uploadData.licence.part_one}/${uploadData.licence.part_two}`
        ).should('exist')
      })

    cy.findByText('Document')
      .parent()
      .parent()
      .within(() => {
        cy.findByText('Document name')
          .parent()
          .within(() => {
            cy.findByText(fileName).should('exist')
          })
      })

    const licence_str = `${uploadData.licence.type} ${uploadData.licence.part_one}/${uploadData.licence.part_two}`
    const product_title = `${uploadData.brand} ${uploadData.strength} ${uploadData.doseForm} - ${licence_str}`.toUpperCase()

    cy.findByText(product_title)
      .parent()
      .within(() => {
        cy.findAllByText('Change').last().click()
      })

    cy.findAllByText('New Public Assessment Report')
      .not('title')
      .should('have.length', 1)

    cy.findByLabelText('Brand/Generic name').should(
      'have.value',
      uploadData.brand
    )
  })
  it('shows the uploaded file when going back to upload file page', () => {
    startNewParForm()
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    completeUploadForm(uploadData, uploadPageTitle)

    const fileName = 'rabbit-anti-human-stuff.pdf'
    const expectedTitle = 'Upload your PDF'
    completeUploadFile(fileName, expectedTitle)

    cy.findByText('Document')
      .parent()
      .within(() => {
        cy.findAllByText('Change').last().click()
      })

    cy.findByText(`Current file`).should('exist')
    cy.findByText(`Upload new file instead`).should('exist')
    cy.findByText(fileName).should('exist')

    cy.findAllByText('Continue').last().click()
    cy.findByText('Current file').should('exist')

    cy.findAllByText('Continue').first().click()
    cy.findAllByText('Check your answers before sending the report')
      .not('title')
      .should('have.length', 1)
  })
  it('can submit the form sucessfully', () => {
    if (parsUrl) {
      cy.log('Mocking form submissions endpoint')

      cy.server()

      mockSuccessfulSubmission(baseUrl, parsUrl)
    }

    startNewParForm()

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    completeUploadForm(uploadData, uploadPageTitle)

    const fileName = 'rabbit-anti-human-stuff.pdf'
    const expectedTitle = 'Upload your PDF'
    completeUploadFile(fileName, expectedTitle)

    cy.findAllByText('Check your answers before sending the report')
      .not('title')
      .should('have.length', 1)

    cy.findByText('Accept and send').click()

    cy.findAllByText('Submission complete')
      .not('title')
      .should('have.length', 1)

    cy.findByText('Submit another report').click()

    cy.findByText('What are you doing today?').should('exist')
  })
})
