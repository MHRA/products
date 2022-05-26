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

setUp()

const parsUrl = Cypress.env('PARS_UPLOAD_URL')

describe('New PARs upload', () => {
  beforeEach(() => {
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

  it('can add and delete multiple substances', () => {
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
  it('upload field only accepts PDFs', () => {
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
  it('duplicate licence numbers are not allowed', () => {
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substance: 'Ibuprofen',
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    let uploadPageTitle = 'New Public Assessment Report'
    addDuplicateLicenceNumbers(uploadData, uploadPageTitle)
  })
  it('review page shows the correct information', () => {
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

    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('Check your answers before sending the report')
      })

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
    const product_title =
      `${uploadData.brand} ${uploadData.strength} ${uploadData.doseForm} - ${licence_str}`.toUpperCase()

    cy.findByText(product_title)
      .parent()
      .within(() => {
        cy.findAllByText('Change').last().click()
      })

    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('New Public Assessment Report')
      })

    cy.findByLabelText('Brand/Generic name').should(
      'have.value',
      uploadData.brand
    )
  })
  it('shows the uploaded file when going back to upload file page', () => {
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
    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('Check your answers before sending the report')
      })
  })
  it('can submit the form sucessfully', () => {
    if (parsUrl) {
      cy.log('Mocking form submissions endpoint')

      mockSuccessfulSubmission(parsUrl)
    }

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

    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('Check your answers before sending the report')
      })

    cy.findByText('Accept and send').click()

    cy.get('body')
      .find('h1')
      .should((h1) => {
        expect(h1).to.contain('Submission complete')
      })

    cy.findByText('Submit another report').click()

    cy.findByText('What are you doing today?').should('exist')
  })
})
