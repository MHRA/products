import Document, { Html, Head, Main, NextScript } from 'next/document';
import { Header } from '../header';
import { Footer } from '../footer';

class MyDocument extends Document {
  static async getInitialProps(ctx) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }

  render() {
    return (
      <Html
        lang="en"
        className="govuk-template js history flexbox no-flexboxtweener fixedsticky-withoutfixedfixed"
      >
        <Head />

        <body className="govuk-template__body">
          <a href="#main-content" className="govuk-skip-link">
            Skip to main content
          </a>

          <Header />

          <Main />

          <Footer />

          <NextScript />
        </body>
      </Html>
    );
  }
}

export default MyDocument;
