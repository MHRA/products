const reportValidityPolyfill = () => {
  if (
    global &&
    global.window &&
    !global.window.HTMLInputElement.prototype.reportValidity
  ) {
    console.log('Initiating reportValidity polyfill')
    global.window.HTMLInputElement.prototype.reportValidity = function () {
      if (this.checkValidity()) {
        return true
      } else {
        alert(this.validationMessage)
        return false
      }
    }
  }
}

reportValidityPolyfill()
