import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../providers/AuthProvider';

export default function OAuthCallbackPage() {
  const { oAuthLogin } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    const hash = new URLSearchParams(window.location.hash.slice(1));
    const accessToken = hash.get('access_token');

    if (accessToken) {
      oAuthLogin(accessToken);
    }
    navigate('/');
  }, []);

  return <>You're not supposed to see this...</>;
}
