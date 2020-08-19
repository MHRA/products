import Head from 'next/head';
import Styled from 'styled-components';
import { nibscGreen } from '../styles/colors';

const H1 = Styled.h1`
  color: ${nibscGreen};
`;

export default function Home() {
  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <H1>Welcome to NIBSC!</H1>
        <img src="/images/NIBSC_logo.png" />
      </main>

      <footer>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by <img src="/vercel.svg" alt="Vercel Logo" />
        </a>
      </footer>
    </div>
  );
}
