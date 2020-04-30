import React from "react"
import Cookies from "universal-cookie"


import Layout from "../components/Layout"
import SEO from "../components/SEO"
import styled from "styled-components"
import { mhraWhite, primaryColor, mhra } from "../utils/colors"


const StyledCookiePolicy = styled.section`
  form {
    label {
      padding-top: 0;
      display: inline-block;
      padding: 40px 50px 40px 0;
      input {
        margin-right: 30px;
      }
    }

    button {
      appearance: none;
      color: ${mhraWhite};
      background-color: ${primaryColor};
      border-radius: 5px;
      border: 1px solid ${mhra};
      display: block;
      padding: 10px 20px;
      cursor: pointer;
    }
  }
`;



class CookiePolicy extends React.Component {

  constructor(props) {
    super(props);
    this.state = {
      cookiesAllowed: true  // This will be updated once the page is loaded.
    };
  }
  
  componentDidMount() {
    this.setState({
      cookiesAllowed: window.localStorage.getItem("showCookieBanner") === "false"
    });
  }

  deleteAllStorage() {
    window.localStorage.clear();
    window.sessionStorage.clear();
    const cookies = new Cookies();
    for (const cookieName of Object.keys(cookies.getAll())) {
      cookies.remove(cookieName);
    }
  }

  handleCookieFormSubmit = (e) => {
    e.preventDefault();

    if (this.state.cookiesAllowed) {
      window.localStorage.setItem("showCookieBanner", "false")
    } else {
      this.deleteAllStorage();
    }

    // Navigate to the home page. Seems to be the only way to stop analytics.
    window.location.href = '/';
  }

  handleCookiesOn = () => {
    this.setState({
      cookiesAllowed: true
    });
  }

  handleCookiesOff = () => {
    this.setState({
      cookiesAllowed: false
    });
  }
  

  render() {
    const title = `Learning Modules for Continuous Professional Development`
    return (
      <StyledCookiePolicy>
        <Layout title={title}>
          <SEO title={title} />
          <h2>Cookie policy</h2>
          <p>Cookies are files saved on your phone, tablet or computer when you visit a website.</p>
          <p>We use cookies to store information about how you use the Learning Modules for Continuous Professional Development website, such as the pages you visit.</p>
          <p>You can find out more about <a href="https://ico.org.uk/your-data-matters/online/cookies/">how to manage cookies</a> on the Information Commissioner’s Office (ICO) website.</p>
          <h3>Necessary cookies</h3>
          <p>We use necessary cookies to make our website work. These enable core functionality such as security, network management and accessibility.</p>

          <h3>Google Analytics cookies</h3>

          <p>We use Google Analytics, a third party service, to collect standard internet log information and details of visitor behaviour patterns. We do this to find out such things as the number of visitors to the various parts of the site, the search terms used and geographic region.</p>

          <p>This information is only processed in a way that does not identify anyone. We do not make, and do not allow Google to make, any attempt to find out the identities of those visiting our website. Please visit Google’s <a href="https://support.google.com/analytics/answer/6004245">overview of privacy and safeguarding data</a> to know more about their policies.</p>
          
          <p>You can turn these cookies on or off below, and then save your preferences. This site stores your acceptance of Google Analytics cookies in your browser’s local storage as 'allowStorage'.</p>
      <form onSubmit={this.handleCookieFormSubmit}>
            <p>
              <label htmlFor="cookie-on">
                <input
                  type="radio"
                  name="cookie"
                  id="cookie-on"
                  value="on"
                  onChange={this.handleCookiesOn}
                  checked={this.state.cookiesAllowed}
                  role="button"
                />
                On
              </label>
              <label htmlFor="cookie-off">
                <input
                  type="radio"
                  name="cookie"
                  id="cookie-off"
                  value="off"
                  onChange={this.handleCookiesOff}
                  checked={!this.state.cookiesAllowed}
                  role="button"
                />
                Off
              </label>
            </p>
            <p><b>Google Analytics cookies we use are:</b></p>
            <table>
              <tbody>
                <tr>
                  <th scope="col">Name</th>
                  <th scope="col">Expiration</th>
                  <th scope="col">Description</th>
                </tr>
                <tr>
                  <td>_ga</td>
                  <td>2 years</td>
                  <td>Used to distinguish users.</td>
                </tr>
                <tr>
                  <td>_gid</td>
                  <td>24 hours</td>
                  <td>Used to distinguish users.</td>
                </tr>
                <tr>
                  <td>_gat</td>
                  <td>1 minute</td>
                  <td>Used to limit the number of messages which are sent.</td>
                </tr>
              </tbody>
            </table>
            <p>
              <button>Save your preferences</button>
            </p>
          </form>
        </Layout>
      </StyledCookiePolicy>
    );
  }
}

export default CookiePolicy;
 
