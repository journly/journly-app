import { Paper } from '@mui/material';
import React, { ReactNode } from 'react';

interface SectionProps {
  title?: string;
  children: ReactNode;
  additionalStyle?: string;
}

const Section: React.FC<SectionProps> = ({ title, children, additionalStyle }) => {
  return (
    <Paper elevation={0} className={'bg-white border border-gray-200 rounded-lg p-7 ' + additionalStyle}>
      {title && <h1>{title}</h1>}
      <main>{children}</main>
    </Paper>
  );
};

export default Section;