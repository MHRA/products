import Head from 'next/head';
import Styled from 'styled-components';
import { nibscMainGreen } from '../styles/colors';
import Page from '../components/page';

const H1 = Styled.h1`
  color: ${nibscMainGreen};
`;

export default function Home() {
  return (
    <Page title="Homepage">

        <H1>Welcome to NIBSC!</H1>
        <img src="/images/NIBSC_logo.png" />


      <footer>
      
      </footer>
    </Page>
  );
}
