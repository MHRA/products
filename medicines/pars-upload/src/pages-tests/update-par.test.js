/**
 * @jest-environment jsdom
 */

import { getIdOfParToUpdate, combineFormDatas } from '../pages/update-par'

describe(getIdOfParToUpdate, () => {
  it('extracts ID of par to update', () => {
    let productStep = { type: 'product', data: new FormData() }
    let fileStep = { type: 'file', data: new FormData() }
    let reviewStep = { type: 'review', data: new FormData() }
    let getParStep = { type: 'get_par', data: new FormData() }
    let parId = '17dfsjkl12kljdfk129'
    getParStep.data.append('par_url', `https://blob.net/${parId}`)

    let id = getIdOfParToUpdate([productStep, fileStep, reviewStep, getParStep])
    expect(id).toBe(parId)
  })
})

describe(combineFormDatas, () => {
  it('combines form data', () => {
    let productStep = { type: 'product', data: new FormData() }
    productStep.data.append('product_field_1', 'product_value_1')
    productStep.data.append('product_field_2', 'product_value_2')

    let fileStep = { type: 'file', data: new FormData() }
    fileStep.data.append('file_field_1', 'file_value_1')
    fileStep.data.append('file_field_2', 'file_value_2')

    let reviewStep = { type: 'review', data: new FormData() }
    reviewStep.data.append('review_field_1', 'review_value_1')
    reviewStep.data.append('review_field_2', 'review_value_2')

    let combinedData = combineFormDatas([productStep, fileStep, reviewStep])
    expect(Array.from(combinedData.keys()).length).toBe(6)
    expect(combinedData.get('product_field_1')).toBe('product_value_1')
    expect(combinedData.get('file_field_1')).toBe('file_value_1')
    expect(combinedData.get('review_field_1')).toBe('review_value_1')
  }),
    it('ignores update PAR step', () => {
      let productStep = { type: 'product', data: new FormData() }
      productStep.data.append('product_field_1', 'product_value_1')
      productStep.data.append('product_field_2', 'product_value_2')

      let getParStep = { type: 'get_par', data: new FormData() }
      getParStep.data.append('get_field_1', 'get_value_1')
      getParStep.data.append('get_field_2', 'get_value_2')

      let combinedData = combineFormDatas([productStep, getParStep])
      expect(Array.from(combinedData.keys()).length).toBe(2)
      expect(combinedData.get('get_field_1')).toBe(null)
      expect(combinedData.get('product_field_1')).toBe('product_value_1')
    })
})
