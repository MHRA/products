const popup = require('./authPopup')

test('extracts base url from url', () => {
  expect(popup.getCurrentHost('http://theserver/1/2/3/4/5')).toBe(
    'http://theserver'
  )
})

test('extracts base url from a url with port and path', () => {
  expect(popup.getCurrentHost('http://localhost:3000/1/2/3/4/5')).toBe(
    'http://localhost:3000'
  )
})

test('extracts base url from a url without path', () => {
  expect(popup.getCurrentHost('http://theserver:8000')).toBe(
    'http://theserver:8000'
  )
})
