/* eslint-env node, mocha */

export const setUp = () => {
  Cypress.on('window:before:load', (win) => {
    // Clear out session storage so that the disclaimer is always presented.
    win.sessionStorage.clear()
  })
}

export const mockSuccessfulSubmission = (url) => {
  cy.intercept('POST', url, {
    statusCode: 200,
    fixture: 'mock_submission_success',
  })
}

export const completeFindParToUpdateStep = (parUrl) => {
  cy.get('body')
    .find('h1')
    .should((h1) => {
      expect(h1).to.contain('Search for an existing Public Assessment Report')
    })
  cy.findByLabelText('Please insert URL').type(parUrl)
  cy.findByText('Continue').click()
}

export const addAndDeleteSubstances = (uploadData, expectedTitle) => {
  cy.findAllByText(expectedTitle).not('title').should('exist')

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

export const addAndDeleteProducts = (uploadData, expectedTitle) => {
  cy.findAllByText(expectedTitle).not('title').should('exist')

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
  const product_title =
    `${uploadData.brand} ${uploadData.strength} ${uploadData.doseForm} - ${licence_str}`.toUpperCase()

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

export const addDuplicateLicenceNumbers = (uploadData, expectedTitle) => {
  cy.findAllByText(expectedTitle).not('title').should('exist')

  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substance)

  cy.findByText('Licence number')
    .parent()
    .parent()
    .within(() => {
      cy.findByLabelText('Type').select(uploadData.licence.type)
      cy.findByLabelText('First five digits').type(uploadData.licence.part_one)
      cy.findByLabelText('Last four digits').type(uploadData.licence.part_two)
    })
  cy.findByText('Add another product').click()

  cy.findByLabelText('Brand/Generic name').should('have.value', '')

  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substance)

  cy.findByText('Licence number')
    .parent()
    .parent()
    .within(() => {
      cy.findByLabelText('Type').select(uploadData.licence.type)
      cy.findByLabelText('First five digits').type(uploadData.licence.part_one)
      cy.findByLabelText('Last four digits').type(uploadData.licence.part_two)
    })

  cy.findByText('Add another product').click()

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

export const completeUploadForm = (uploadData, expectedTitle) => {
  cy.findAllByText(expectedTitle).not('title').should('exist')

  cy.findByLabelText('Brand/Generic name').type(uploadData.brand)

  cy.findByLabelText('Strength').type(uploadData.strength)

  cy.findByLabelText('Pharmaceutical dose form').type(uploadData.doseForm)

  cy.findByLabelText('Active substance(s)').type(uploadData.substances[0])

  cy.findByText('Add another active substance').click()
  cy.findAllByLabelText('Active substance(s)')
    .last()
    .type(uploadData.substances[1])

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

export const completeUploadFile = (
  fileName,
  expectedTitle,
  mimeType = 'application/pdf',
  encoding = ''
) => {
  cy.findAllByText(expectedTitle).not('title').should('exist')

  cy.fixture(fileName).then((fileContent) => {
    // The `upload` method is provided by https://github.com/abramenal/cypress-file-upload/tree/v3.5.3
    cy.get('input[type=file]').attachFile({
      fileContent,
      fileName,
      mimeType,
      encoding,
    })
  })

  cy.findByText('Continue').click()
}
