import { useEffect } from "react";
import { useAuth } from "../../providers/AuthProvider";
import { useNavigate } from "react-router-dom";

export default function OAuthCallbackPage() {
  const { oAuthLogin } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    const hash = new URLSearchParams(window.location.hash.slice(1));
    const accessToken = hash.get('access_token');
    const refreshToken = hash.get('refresh_token');


    if (accessToken == null || refreshToken == null) {
      console.log("gay")
    }
    else {
      oAuthLogin(accessToken, refreshToken);
      navigate("/");
    }
  }, [])

  return <>hi</>
}
