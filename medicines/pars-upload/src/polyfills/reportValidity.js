const reportValidityPolyfill = () => {
  if (!HTMLInputElement.prototype.reportValidity) {
    HTMLInputElement.prototype.reportValidity = function () {
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
