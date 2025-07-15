import { useEffect, useRef, useState } from "react";
import { Navigate, useNavigate } from "react-router-dom";
import { AuthStatus, useAuth } from "../providers/AuthProvider";

export function VerifyPage() {
  const { logout, resendVerificationCode, verifyEmail, checkAuthenticated } = useAuth();
  const [code, setCode] = useState<string[]>(Array(6).fill(''));
  const [resendCooldown, setResendCooldown] = useState<number>(30);
  const [verificationFailed, setVerificationFailed] = useState(false);
  const resendIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const inputsRef = useRef<Array<HTMLInputElement | null>>([]);
  let navigate = useNavigate();
  const [authStatus, setAuthStatus] = useState<AuthStatus | null>(null);

  const onVerify = async (code: string) => {
    let result = await verifyEmail(Number(code));

    if (!result) {
      setVerificationFailed(true);
      return
    }

    navigate("/");
  }


  useEffect(() => {
    if (resendCooldown === 0 && resendIntervalRef.current) {
      clearInterval(resendIntervalRef.current);
      resendIntervalRef.current = null;
    }
  }, [resendCooldown]);

  useEffect(() => {
    if (resendCooldown > 0 && !resendIntervalRef.current) {
      resendIntervalRef.current = setInterval(() => {
        setResendCooldown((prev) => prev - 1);
      }, 1000);
    }

    return () => {
      if (resendIntervalRef.current) {
        clearInterval(resendIntervalRef.current);
        resendIntervalRef.current = null;
      }
    };
  }, []);

  const handleChange = (index: number, value: string) => {
    if (!/^[0-9]?$/.test(value)) return;

    const newCode = [...code];
    newCode[index] = value;
    setCode(newCode);
    setVerificationFailed(false);

    if (value && index < 5) {
      inputsRef.current[index + 1]?.focus();
    }
  };

  const handleResend = async () => {
    if (resendCooldown > 0) return;

    await resendVerificationCode();

    console.log("resend")

    setResendCooldown(30);

    resendIntervalRef.current = setInterval(() => {
      setResendCooldown((prev) => prev - 1);
    }, 1000);
  };

  const handleKeyDown = (index: number, e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Backspace' && !code[index] && index > 0) {
      const newCode = [...code];
      newCode[index - 1] = '';
      setCode(newCode);
      inputsRef.current[index - 1]?.focus();
    }
  };

  const handlePaste = (e: React.ClipboardEvent<HTMLInputElement>) => {
    e.preventDefault();
    const pasted = e.clipboardData.getData('text').replace(/\D/g, '').slice(0, 6);
    const newCode = [...code];
    for (let i = 0; i < pasted.length; i++) {
      newCode[i] = pasted[i];
    }
    setCode(newCode);
    inputsRef.current[pasted.length - 1]?.focus();
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const codeStr = code.join('');
    if (codeStr.length === 6) {
      onVerify(codeStr);
    }
  };

  const backToLoginHandler = async () => {
    await logout();

    navigate("/login")
  }

  useEffect(() => {
    let cancelled = false;

    const verify = async () => {
      const res = await checkAuthenticated();

      if (!cancelled) setAuthStatus(res);
    };

    verify();

    return () => {
      cancelled = true;
    };
  }, [checkAuthenticated]);

  if (authStatus == AuthStatus.Authenticated) {
    return <Navigate to="/" replace />;
  }

  if (authStatus == AuthStatus.Unauthenticated) {
    return <Navigate to="/login" replace />;
  }


  return (
    <div className="w-full h-screen flex justify-center items-center ">
      <div className="max-w-md mx-auto mt-20 p-6 border rounded-2xl shadow-lg bg-white">
        <h1 className="text-2xl font-semibold text-center mb-4">Verify Your Email</h1>
        <p className="text-gray-600 text-center mb-6">
          Enter the 6-digit verification code we sent to your email.
        </p>
        <form onSubmit={handleSubmit}>
          <div className="flex justify-between gap-2 mb-6">
            {code.map((digit, index) => (
              <input
                key={index}
                type="text"
                inputMode="numeric"
                maxLength={1}
                value={digit}
                ref={(el) => (inputsRef.current[index] = el)}
                onChange={(e) => handleChange(index, e.target.value)}
                onKeyDown={(e) => handleKeyDown(index, e)}
                onPaste={handlePaste}
                className={`w-12 h-12 border rounded-lg text-center text-lg font-mono shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 ${verificationFailed ? "border-red-500" : ""}`}
              />
            ))}
          </div>
          <button
            type="submit"
            className="w-full py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition mb-3"
            disabled={code.join('').length < 6}
          >
            Verify
          </button>
          <div className="text-center text-sm text-gray-600">
            Didn’t get the code?{' '}
            <button
              type="button"
              onClick={handleResend}
              disabled={resendCooldown > 0}
              className={`font-semibold ${resendCooldown === 0 ? 'text-blue-600 hover:underline' : 'text-gray-400 cursor-not-allowed'
                }`}
            >
              {resendCooldown === 0 ? 'Resend code' : `Resend in ${resendCooldown}s`}
            </button>
          </div>
        </form>
        <div onClick={backToLoginHandler}>
          <p className="text-blue-600 font-semibold cursor-pointer mt-3">Back to login</p>
        </div>
      </div>
    </div>
  );
}
