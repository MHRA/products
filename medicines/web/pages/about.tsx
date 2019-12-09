import React from 'react';
import Page from '../components/page';

const App: React.FC = () => {
  return (
    <Page title="About">
      <h2>SPC-PILs</h2>
      <p>
        Patient information leaflet (PILs) are found in every medicine pack, and
        provide information on how a medicine should be used safely.
      </p>
      <p>
        Summaries of Product Characteristics (SPCs) are a description of a
        medicinal product’s properties and the conditions attached to its use. A
        PIL will be based on an SPC.
      </p>
      <p>
        We publish the most up-to-date information for a medicine according to
        its licence history.
      </p>
      <p>
        PILs can be updated several times during the product’s lifecycle,so the
        leaflet a patient gets with their medicine could be different to the one
        found here.
      </p>
      <p>
        We hold data for medicines licensed at a national (UK) level. Some
        medicines are licensed centrally by the European Medicines Agency (EMA).
        For product information on these medicines,{' '}
        <a href="https://www.ema.europa.eu/en/medicines">use the EMA website</a>
        .
      </p>
      <p>
        Once a marketing authorisation has been cancelled, the product
        information will stay on the website for a year.
      </p>
      <p>
        Market authorisation holders have 6 months to exhaust stocks after they
        cancel their product. This means the PILs will still be available on the
        website during this period, with an extra 6 months for patients who
        still have the product at home.
      </p>
      <p>
        If you have any questions or comments on this list of product
        information, please{' '}
        <a href="https://www.gov.uk/guidance/contact-mhra#customer-services">
          contact our Customer Services Team
        </a>
        .
      </p>
      <h2>PARs</h2>
      <p>
        We make scientific assessment reports called PARs (Public Assessment
        Reports) for new marketing authorisations granted after 30 October 2005.
      </p>
      <p>
        We also publish PARs for all marketing authorisation applications that
        were refused after 1 April 2019.
      </p>
      <p>
        PARs are published and edited in accordance with a specific EC
        Directive, 2004/27/EC.
      </p>
      <p>
        For some medicines licensed at a European level, the PARs are prepared
        by either the European Medicines Agency (EMA) or the responsible
        authority of another EU member state. The PARs for these medicines can
        be found on the following websites:
      </p>
      <ul>
        <li>
          <a href="https://www.ema.europa.eu/en/medicines/field_ema_web_categories%253Aname_field/Human/ema_group_types/ema_medicine">
            European Medicines Agency (EMA)
          </a>
        </li>
        <li>
          <a href="http://mri.medagencies.org/Human/">
            Head of Medicines Agency
          </a>
        </li>
      </ul>
      <h2>Changes to PARs</h2>
      <p>
        Subsequent non-safety changes (variations) of clinical significance for
        each marketing authorisation are provided at the end of each PAR as
        separate annexes.
      </p>
      <p>
        Minor changes to a marketing authorisation, for example, changes in pack
        sizes and minor updates to the product literature, may not be
        represented in the PAR.
      </p>
      <p>
        We also publish{' '}
        <a href="http://www.mhra.gov.uk/safety-public-assessment-reports/index.htm">
          Safety Public Assessment Reports
        </a>
        , which provide details of significant safety changes to a marketing
        authorisation.
      </p>
    </Page>
  );
};

export default App;
