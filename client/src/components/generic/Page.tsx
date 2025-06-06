import React, { ReactNode } from 'react';

interface PageProps {
  title?: string;
  children: ReactNode;
}

const PageWrapper: React.FC<PageProps> = ({ title, children }) => {
  return (
    <div className='p-4 pt-6 max-w-screen-2xl mx-auto'>
      {title && <h1>{title}</h1>}
      <main>{children}</main>
    </div>
  );
};

export default PageWrapper;