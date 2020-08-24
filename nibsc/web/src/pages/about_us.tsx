import Link from 'next/link';
import Page from '../components/page';

import { H1 } from '../components/typography';

const AboutUs: React.FC = () => {
  return (
    <Page title="About us">
      <H1>About us</H1>
      <nav>
        <ul>
          <li>
            <Link href="">
              <a>Control testing of biological medicines</a>
            </Link>
          </li>
          <li>
            <Link href="">
              <a>Biological standards and reference materials</a>
            </Link>
          </li>
          <li>
            <Link href="">
              <a>Applied research</a>
            </Link>
          </li>
          <li>
            <Link href="">
              <a>Our work with the World Health Organisation (WHO)</a>
            </Link>
          </li>
          <li>
            <Link href="">
              <a>NIBSC Management</a>
            </Link>
          </li>
        </ul>
      </nav>
    </Page>
  );
};

export default AboutUs;
