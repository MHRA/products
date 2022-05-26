import { getCurrentHost } from './authPopup'

describe.each([
  ['https://website/1/2/3/4/5', 'https://website'],
  ['http://localhost:4000/1/2/3/4/5', 'http://localhost:4000'],
  ['http://website', 'http://website'],
  ['https://website.com/1/2/3/4/5', 'https://website.com'],
])('', (input, expected) => {
  test(`getCurrentHost for ${input} returns ${expected}`, () => {
    expect(getCurrentHost(input)).toBe(expected)
  })
})
