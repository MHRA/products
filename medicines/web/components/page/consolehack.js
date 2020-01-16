export default `/* Inspired by https://github.com/cypress-io/cypress/issues/448#issuecomment-393486805 */

(function() {
  var console = (function(oldCons){
    return {
      log: function(text){
        const logElement = document.getElementById("loggity");
        if (logElement) {
          logElement.innerHTML += "\\nLOG: " + text;
        }
        
        oldCons.log(text);
      },
      info: function (text) {
        const logElement = document.getElementById("loggity");
        if (logElement) {
          logElement.innerHTML += "\\nINFO: " + text;
        }
        oldCons.info(text);
      },
      warn: function (text) {
        const logElement = document.getElementById("loggity");
        if (logElement) {
          logElement.innerHTML += "\\nWARN: " + text;
        }
        oldCons.warn(text);
      },
      error: function (text) {
        const logElement = document.getElementById("loggity");
        if (logElement) {
          logElement.innerHTML += "\\nERROR: " + text;
        }
        oldCons.error(text);
      }
    };
  }(window.console));

  if (true || window.envConfig.cypressConsole == 'true') {
    window.console = console;
  }

  console.log('Console initialised.');
})();

`;