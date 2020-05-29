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
  it('can get to the form page', () => {
    cy.server()

    cy.visit('/')

    cy.findByText('Upload a new document').click()

    cy.findByText('Continue').click()

    cy.findAllByText('New Public Assessment Report')
      .not('title')
      .should('have.length', 1)
  })
})

describe('PARs upload form', () => {
  it('can add and delete multiple substances', () => {
    cy.visit('/new-par')

    cy.findByLabelText('Brand/Generic name').type('Ibuprofen pills')

    cy.findByLabelText('Strength').type('Really powerful stuff')

    cy.findByLabelText('Pharmaceutical dose form').type('some form')

    cy.findByLabelText('Active substance(s)').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance(s)').last().type('Paracetamol')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance(s)').last().type('Temazepam')

    cy.findAllByLabelText('Active substance(s)').should('have.length', 3)

    cy.findAllByText('Delete substance').eq(1).parent('button').click()

    cy.findAllByLabelText('Active substance(s)').should('have.length', 2)

    cy.findAllByLabelText('Active substance(s)')
      .eq(0)
      .should('have.value', 'Ibuprofen')

    cy.findAllByLabelText('Active substance(s)')
      .eq(1)
      .should('have.value', 'Temazepam')
  })

  it('can add and delete multiple products', () => {
    const productName = 'Ibuprofen pills'
    const strength = 'Really powerful stuff'
    const dose = 'some form'
    const licence = { type: 'THR', part_one: '12345', part_two: '6789' }

    cy.visit('/new-par')

    cy.findByLabelText('Brand/Generic name').type(productName)

    cy.findByLabelText('Strength').type(strength)

    cy.findByLabelText('Pharmaceutical dose form').type(dose)

    cy.findByLabelText('Active substance(s)').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance(s)').last().type('Paracetamol')

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select(licence.type)
        cy.findByLabelText('First five digits').type(licence.part_one)
        cy.findByLabelText('Last four digits').type(licence.part_two)
      })

    cy.findByText('Add another product').click()

    // Form should now be blank and ready for entering another product
    cy.findByLabelText('Brand/Generic name').should('have.value', '')

    const licence_str = `${licence.type} ${licence.part_one}/${licence.part_two}`
    const product_title = `${productName}, ${strength}, ${dose}, ${licence_str}`

    cy.findByText(product_title)
      .parent()
      .within(() => {
        cy.findByText('Edit').click()
      })

    cy.findByLabelText('Brand/Generic name').should('have.value', productName)

    cy.findByText(product_title)
      .parent()
      .within(() => {
        cy.findByText('Remove').click()
      })

    cy.findByLabelText('Brand/Generic name').should('have.value', '')
  })

  it('review page shows the correct information', () => {
    const productName = 'Ibuprofen pills'
    const strength = 'Really powerful stuff'
    const dose = 'some form'

    const licence = {
      type: 'THR',
      part_one: '12345',
      part_two: '6789',
    }

    cy.visit('/new-par')

    cy.findByLabelText('Brand/Generic name').type(productName)

    cy.findByLabelText('Strength').type(strength)

    cy.findByLabelText('Pharmaceutical dose form').type(dose)

    cy.findByLabelText('Active substance(s)').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance(s)').last().type('Paracetamol')

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select(licence.type)
        cy.findByLabelText('First five digits').type(licence.part_one)
        cy.findByLabelText('Last four digits').type(licence.part_two)
      })

    cy.findByText('Continue').click()

    cy.findAllByText('Upload your PDF').not('title').should('have.length', 1)

    const fileName = 'rabbit-anti-human-stuff.pdf'

    cy.fixture(fileName).then((fileContent) => {
      // The `upload` method is provided by https://github.com/abramenal/cypress-file-upload/tree/v3.5.3
      cy.get('input[type=file]').upload({
        fileContent,
        fileName,
        mimeType: 'application/pdf',
      })
    })

    cy.findByText('Continue').click()

    cy.findAllByText('Check your answers before sending the report')
      .not('title')
      .should('have.length', 1)

    cy.findByText('Brand/Generic name')
      .parent()
      .within(() => {
        cy.findByText(productName).should('exist')
      })

    cy.findByText('Strength')
      .parent()
      .within(() => {
        cy.findByText(strength).should('exist')
      })

    cy.findByText('Pharmaceutical dose form')
      .parent()
      .within(() => {
        cy.findByText(dose).should('exist')
      })

    cy.findByText('Active substances')
      .parent()
      .within(() => {
        cy.findByText('Ibuprofen, Paracetamol').should('exist')
      })

    cy.findByText('Licence number')
      .parent()
      .within(() => {
        cy.findByText(
          `${licence.type} ${licence.part_one}/${licence.part_two}`
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

    const licence_str = `${licence.type} ${licence.part_one}/${licence.part_two}`

    cy.findByText(`${productName}, ${strength}, ${dose}, ${licence_str}`)
      .parent()
      .within(() => {
        cy.findByText('Change').click()
      })

    cy.findAllByText('New Public Assessment Report')
      .not('title')
      .should('have.length', 1)

    cy.findByLabelText('Brand/Generic name').should('have.value', productName)
  })

  it('can submit the form sucessfully', () => {
    cy.server()

    mockSuccessfulSubmission()

    cy.visit('/new-par')

    cy.findByLabelText('Brand/Generic name').type('Ibuprofen pills')

    cy.findByLabelText('Strength').type('Really powerful stuff')

    cy.findByLabelText('Pharmaceutical dose form').type('some form')

    cy.findByLabelText('Active substance(s)').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance(s)').last().type('Paracetamol')

    const licence = { type: 'THR', part_one: '12345', part_two: '6789' }

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select(licence.type)
        cy.findByLabelText('First five digits').type(licence.part_one)
        cy.findByLabelText('Last four digits').type(licence.part_two)
      })

    cy.findByText('Continue').click()

    cy.findAllByText('Upload your PDF').not('title').should('have.length', 1)

    const fileName = 'rabbit-anti-human-stuff.pdf'

    cy.fixture(fileName).then((fileContent) => {
      // The `upload` method is provided by https://github.com/abramenal/cypress-file-upload/tree/v3.5.3
      cy.get('input[type=file]').upload({
        fileContent,
        fileName,
        mimeType: 'application/pdf',
      })
    })

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
