let polyfill

// grab fetch polyfill from remote URL, could be also from a local package
before(() => {
  const polyfillUrl = 'https://unpkg.com/unfetch/dist/unfetch.umd.js'

  cy.request(polyfillUrl).then((response) => {
    polyfill = response.body
  })
})

const mockSuccessfulSubmission = () =>
  cy.route(
    'POST',
    'http://localhost:8000/pars',
    'fixture:mock_submission_results.json'
  )

describe('Home page', () => {
  it('can get to the form page', () => {
    cy.server()

    cy.visit('/')

    cy.findByText('Upload a new document').click()

    cy.findByText('Continue').click()

    cy.findByText('New Public Assessment Report').should('exist')
  })
})

describe('PARs upload form', () => {
  it('can add multiple substances', () => {
    cy.visit('/new-par')

    cy.findByLabelText('Product name').type('Ibuprofen pills')

    cy.findByLabelText('Strength').type('Really powerful stuff')

    cy.findByLabelText('Pharmaceutical dose form').type('some form')

    cy.findByLabelText('Active substance').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance').last().type('Paracetamol')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance').last().type('Temazepam')

    cy.findAllByLabelText('Active substance').should('have.length', 3)
  })

  it('can add multiple products', () => {
    const productName = 'Ibuprofen pills'
    const strength = 'Really powerful stuff'
    const dose = 'some form'
    const license = { type: 'THR', part_one: '12345', part_two: '6789' }

    cy.visit('/new-par')

    cy.findByLabelText('Product name').type(productName)

    cy.findByLabelText('Strength').type(strength)

    cy.findByLabelText('Pharmaceutical dose form').type(dose)

    cy.findByLabelText('Active substance').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance').last().type('Paracetamol')

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select(license.type)
        cy.findByLabelText('First five digits').type(license.part_one)
        cy.findByLabelText('Last four digits').type(license.part_two)
      })

    cy.findByText('Add another product').click()

    const license_str = `${license.type} ${license.part_one}/${license.part_two}`

    cy.findByText(
      `${productName}, ${strength}, ${dose}, ${license_str}`
    ).should('exist')
  })

  it('can submit the form sucessfully', () => {
    cy.server()

    mockSuccessfulSubmission()

    cy.visit('/new-par')

    cy.findByLabelText('Product name').type('Ibuprofen pills')

    cy.findByLabelText('Strength').type('Really powerful stuff')

    cy.findByLabelText('Pharmaceutical dose form').type('some form')

    cy.findByLabelText('Active substance').type('Ibuprofen')

    cy.findByText('Add another active substance').click()

    cy.findAllByLabelText('Active substance').last().type('Paracetamol')

    const license = { type: 'THR', part_one: '12345', part_two: '6789' }

    cy.findByText('Licence number')
      .parent()
      .parent()
      .within(() => {
        cy.findByLabelText('Type').select(license.type)
        cy.findByLabelText('First five digits').type(license.part_one)
        cy.findByLabelText('Last four digits').type(license.part_two)
      })

    cy.findByText('Continue').click()

    cy.findByText('Upload your PDF').should('exist')

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

    cy.findByText('Check your answers before sending the report').should(
      'exist'
    )

    cy.findByText('Continue').click()

    cy.findByText('Success!').should('exist')
  })
})
