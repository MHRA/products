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

  it.only('review page shows the correct information', () => {
    const productName = 'Ibuprofen pills'
    const strength = 'Really powerful stuff'
    const dose = 'some form'

    const license = {
      type: 'THR',
      part_one: '12345',
      part_two: '6789',
    }

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

    cy.findByText('License number')
      .parent()
      .within(() => {
        cy.findByText(
          `${license.type} ${license.part_one}/${license.part_two}`
        ).should('exist')
      })

    cy.findByText('Document')
      .parent()
      .within(() => {
        cy.findByText('Document name')
          .parent()
          .within(() => {
            cy.findByText(fileName).should('exist')
          })
      })
  })

  it('can submit the form sucessfully', () => {
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

    cy.findByText('Accept and send').click()

    cy.findByText('Submission complete').should('exist')

    cy.findByText('Submit another report').click()

    cy.findByText('What are you doing today?').should('exist')
  })
})
