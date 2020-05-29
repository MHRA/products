const popup = require('./authPopup')

test('extracts base url from url with port', () => {
  expect(popup.getCurrentHost('http://localhost:3000/1/2/3/4/5')).toBe(
    'http://localhost:3000'
  )
})

test('extracts base url from url', () => {
  expect(popup.getCurrentHost('http://theserver/1/2/3/4/5')).toBe(
    'http://theserver'
  )
})
