import { useNavigate } from 'react-router-dom';
import { Anchor, Image } from '@mantine/core';
import { getGoogleUrl } from '@/utils/getGoogleUrl';
import googleIcon from '../../../assets/icons8-google.svg';
import classes from './GoogleSignInButton.module.css';

export const GoogleSignInButton = () => {
  const navigate = useNavigate();

  return (
    <Anchor className={classes.button} href={getGoogleUrl('/oauth/google')} underline="never">
      <Image src={googleIcon} alt="Google" w={30} h={30} /> Continue with Google
    </Anchor>
  );
};
