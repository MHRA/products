const authPopup = require('./authPopup')

describe.each([
  ['https://website', 'https://website/1/2/3/4/5'],
  ['http://localhost:4000', 'http://localhost:4000/1/2/3/4/5'],
  ['http://website', 'http://website'],
  ['https://website.com', 'https://website.com/1/2/3/4/5'],
])('', (expected, input) => {
  test(`getCurrentHost for ${input} returns ${expected}`, () => {
    expect(authPopup.getCurrentHost(input)).toBe(expected)
  })
})
