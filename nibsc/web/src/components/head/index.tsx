import NextHead from 'next/head';
import React from 'react';

interface IHeadProps {
  title: string;
}

const Head: React.FC<IHeadProps> = (props) => {
  return (
    <NextHead>
      <title>NIBSC - {props.title}</title>
      <meta
        httpEquiv="Content-Security-Policy-Report-Only"
        content="base-uri 'self'; default-src 'self'; script-src 'self'; style-src 'self'; object-src 'none'; form-action 'self'; font-src 'self'; connect-src 'self'; img-src 'self';"
      />
    </NextHead>
  );
};

export default Head;
