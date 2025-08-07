import { ReactNode } from 'react';
import { Text, UnstyledButton } from '@mantine/core';
import classes from './SummaryInfoButton.module.css';

interface SummaryInfoButtonProps {
  children: ReactNode;
  onClick: () => void;
  leftIcon?: ReactNode;
}

export const SummaryInfoButton = ({ children, onClick, leftIcon }: SummaryInfoButtonProps) => {
  return (
    <UnstyledButton onClick={onClick} className={classes.summaryInfoButton}>
      {leftIcon}
      <Text fz="sm">{children}</Text>
    </UnstyledButton>
  );
};
