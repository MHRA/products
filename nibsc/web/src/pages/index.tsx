import Head from 'next/head';
import Styled from 'styled-components';
import { nibscMainGreen } from '../styles/colors';
import Page from '../components/page';

const H1 = Styled.h1`
  color: ${nibscMainGreen};
`;

const Home: React.FC = () => {
  return (
    <Page title="Homepage">
      <footer></footer>
    </Page>
  );
};

export default Home;
