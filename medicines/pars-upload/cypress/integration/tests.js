/* eslint-env node, mocha */

/* global Cypress, cy */

let polyfill

// grab fetch polyfill from remote URL, could be also from a local package
before(() => {
  const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js'

  cy.request(polyfillUrl).then((response) => {
    polyfill = response.body
  })
})

Cypress.on('window:before:load', (win) => {
  delete win.fetch
  // since the application code does not ship with a polyfill
  // load a polyfilled "fetch" from the test
  win.eval(polyfill)
  win.fetch = win.unfetch

  // Clear out session storage so that the disclaimer is always presented.
  win.sessionStorage.clear()
})

const parsUrl = Cypress.env('PARS_UPLOAD_URL')
const baseUrl = Cypress.config().baseUrl

const mockSuccessfulSubmission = () => {
  cy.route({
    method: 'OPTIONS',
    url: parsUrl,
    status: 200,
    headers: {
      'access-control-allow-headers': 'authorization, username',
      'access-control-allow-methods': 'POST',
      'access-control-allow-origin': baseUrl,
      'content-length': '0',
      date: 'Mon, 18 May 2020 16:13:06 GMT',
    },
    response: {},
  })
  cy.route({
    method: 'POST',
    url: parsUrl,
    status: 200,
    response: 'fixture:mock_submission_success.json',
  })
}

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

const addAndDeleteSubstances = (uploadData) => {
  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substance1)

  cy.findByText('Add another active substance').click()

  cy.findAllByLabelText('Active substance(s)')
    .last()
    .type(uploadData.substance2)

  cy.findByText('Add another active substance').click()

  cy.findAllByLabelText('Active substance(s)')
    .last()
    .type(uploadData.substance3)

  cy.findAllByLabelText('Active substance(s)').should('have.length', 3)

  cy.findAllByText('Delete substance').eq(1).parent('button').click()

  cy.findAllByLabelText('Active substance(s)').should('have.length', 2)

  cy.findAllByLabelText('Active substance(s)')
    .eq(0)
    .should('have.value', uploadData.substance1)

  cy.findAllByLabelText('Active substance(s)')
    .eq(1)
    .should('have.value', uploadData.substance3)
}

const addAndDeleteProducts = (uploadData) => {
  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substance1)

  cy.findByText('Add another active substance').click()

  cy.findAllByLabelText('Active substance(s)')
    .last()
    .type(uploadData.substance2)

  cy.findByText('Licence number')
    .parent()
    .parent()
    .within(() => {
      cy.findByLabelText('Type').select(uploadData.licence.type)
      cy.findByLabelText('First five digits').type(uploadData.licence.part_one)
      cy.findByLabelText('Last four digits').type(uploadData.licence.part_two)
    })

  cy.findByText('Add another product').click()

  // Form should now be blank and ready for entering another product
  cy.findByLabelText('Brand/Generic name').should('have.value', '')

  const licence_str = `${uploadData.licence.type} ${uploadData.licence.part_one}/${uploadData.licence.part_two}`
  const product_title = `${uploadData.brand}, ${uploadData.strength}, ${uploadData.doseForm}, ${licence_str}`.toUpperCase()

  cy.findByText(product_title)
    .parent()
    .within(() => {
      cy.findByText('Edit').click()
    })

  cy.findByLabelText('Brand/Generic name').should(
    'have.value',
    uploadData.brand
  )

  cy.findByText(product_title)
    .parent()
    .within(() => {
      cy.findByText('Remove').click()
    })

  cy.findByLabelText('Brand/Generic name').should('have.value', '')
}

const completeUploadForm = (uploadData) => {
  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substances[0])

  for (let i = 1; i < uploadData.substances.length; i++) {
    cy.findByText('Add another active substance').click()
    console.log('SUBSTANCE!! ' + i + ' ' + uploadData.substances[i])
    cy.findAllByLabelText('Active substance(s)')
      .last()
      .type(uploadData.substances[i])
  }

  cy.findByText('Licence number')
    .parent()
    .parent()
    .within(() => {
      cy.findByLabelText('Type').select(uploadData.licence.type)
      cy.findByLabelText('First five digits').type(uploadData.licence.part_one)
      cy.findByLabelText('Last four digits').type(uploadData.licence.part_two)
    })
}

const uploadFile = (fileName) => {
  cy.findAllByText('Upload your PDF').not('title').should('have.length', 1)

  cy.fixture(fileName).then((fileContent) => {
    // The `upload` method is provided by https://github.com/abramenal/cypress-file-upload/tree/v3.5.3
    cy.get('input[type=file]').upload({
      fileContent,
      fileName,
      mimeType: 'application/pdf',
    })
  })
}

describe('PARs upload form', () => {
  it('can add and delete multiple substances', () => {
    cy.visit('/new-par')
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substance1: 'Ibuprofen',
      substance2: 'Paracetamol',
      substance3: 'Temazepam',
    }
    addAndDeleteSubstances(uploadData)
  })

  it('can add and delete multiple products', () => {
    cy.visit('/new-par')
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substance1: 'Ibuprofen',
      substance2: 'Paracetamol',
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    addAndDeleteProducts(uploadData)
  })

  it('review page shows the correct information', () => {
    cy.visit('/new-par')
    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    completeUploadForm(uploadData)
    cy.findByText('Continue').click()

    const fileName = 'rabbit-anti-human-stuff.pdf'
    uploadFile(fileName)
    cy.findByText('Continue').click()

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
    const product_title = `${uploadData.brand}, ${uploadData.strength}, ${uploadData.doseForm}, ${licence_str}`.toUpperCase()

    cy.findByText(product_title)
      .parent()
      .within(() => {
        cy.findByText('Change').click()
      })

    cy.findAllByText('New Public Assessment Report')
      .not('title')
      .should('have.length', 1)

    cy.findByLabelText('Brand/Generic name').should(
      'have.value',
      uploadData.brand
    )
  })

  it('can submit the form sucessfully', () => {
    if (parsUrl) {
      cy.log('Mocking form submissions endpoint')

      cy.server()

      mockSuccessfulSubmission()
    }

    cy.visit('/new-par')

    let uploadData = {
      brand: 'Ibuprofen pills',
      strength: 'Really powerful stuff',
      doseForm: 'some form',
      substances: ['Ibuprofen', 'Paracetamol'],
      licence: { type: 'THR', part_one: '12345', part_two: '6789' },
    }
    completeUploadForm(uploadData)
    cy.findByText('Continue').click()

    const fileName = 'rabbit-anti-human-stuff.pdf'
    uploadFile(fileName)
    cy.findByText('Continue').click()

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
