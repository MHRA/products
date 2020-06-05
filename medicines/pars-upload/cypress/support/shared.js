/* eslint-env node, mocha */

/* global Cypress, cy */

export const setUp = () => {
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
}

export const mockSuccessfulSubmission = (baseUrl, url) => {
  cy.route({
    method: 'OPTIONS',
    url: url,
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
    url: url,
    status: 200,
    response: 'fixture:mock_submission_success.json',
  })
}

export const completeFindParToUpdateStep = (parUrl) => {
  cy.findByLabelText('Please insert URL').type(parUrl)
  cy.findByText('Search').click()
}

export const addAndDeleteSubstances = (uploadData) => {
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

export const addAndDeleteProducts = (uploadData) => {
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
  const product_title = `${uploadData.brand} ${uploadData.strength} ${uploadData.doseForm} - ${licence_str}`.toUpperCase()

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

export const addDuplicateLicenceNumbers = (uploadData) => {
  for (let i = 0; i < 2; i++) {
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
        cy.findByLabelText('First five digits').type(
          uploadData.licence.part_one
        )
        cy.findByLabelText('Last four digits').type(uploadData.licence.part_two)
      })

    cy.findByText('Add another product').click()
  }

  const validationMsg = 'Duplicate licence numbers are not allowed'

  cy.findByText('Licence number')
    .parent()
    .parent()
    .within(() => {
      cy.findByLabelText('Type').then(([el]) => {
        expect(el.validationMessage).to.eq(validationMsg)
      })

      cy.findByLabelText('First five digits').then(([el]) => {
        expect(el.validationMessage).to.eq(validationMsg)
      })

      cy.findByLabelText('Last four digits').then(([el]) => {
        expect(el.validationMessage).to.eq(validationMsg)
      })
    })
}

export const completeUploadForm = (uploadData) => {
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

  cy.findByText('Continue').click()
}

export const completeUploadFile = (fileName) => {
  cy.findAllByText('Upload your PDF').not('title').should('have.length', 1)

  cy.fixture(fileName).then((fileContent) => {
    // The `upload` method is provided by https://github.com/abramenal/cypress-file-upload/tree/v3.5.3
    cy.get('input[type=file]').upload({
      fileContent,
      fileName,
      mimeType: 'application/pdf',
    })
  })

  cy.findByText('Continue').click()
}
