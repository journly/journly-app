import React, { useEffect, useRef, useState } from "react";
import { ModalContainer } from "./ModalContainer";
import { AlertDialog } from "./AlertDialog";
import { useUser } from "../../providers/UserProvider";

interface ChangeEmailModalProps {
  isOpen: boolean;
  onClose: () => void;
  onUpdateSuccess: (email: string) => void;
}

export const ChangeEmailModal: React.FC<ChangeEmailModalProps> = ({ isOpen, onClose, onUpdateSuccess }) => {
  const { updateEmail, validateUserPassword } = useUser();
  const [newEmail, setNewEmail] = useState("");
  const [password, setPassword] = useState("");
  const [changeFailed, setChangeFailed] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");
  const failedTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const root = document.getElementById('root');

  if (!root) return null;

  const handleSubmit = async () => {
    try {
      let res = await validateUserPassword(password);

      if (res) {
        try {
          let successful = await updateEmail(newEmail);

          if (!successful) throw new Error();

          onUpdateSuccess(newEmail);
        } catch {
          setChangeFailed(true);
          setAlertMessage("Invalid email.");
        }
      } else {
        setChangeFailed(true);
        setAlertMessage("Password was incorrect.")
      }

    } catch {
      setChangeFailed(true);
    }
  }

  useEffect(() => {
    if (changeFailed) {
      if (failedTimeoutRef.current) {
        clearTimeout(failedTimeoutRef.current);
        failedTimeoutRef.current = null;
      }

      failedTimeoutRef.current = setTimeout(() => {
        setChangeFailed(false);
      }, 3000);
    }

    return (() => {
      if (failedTimeoutRef.current) {
        clearTimeout(failedTimeoutRef.current)
      }
    })
  }, [changeFailed])

  if (!isOpen) return null;

  return (
    <>
      <ModalContainer onClose={onClose}>
        <h3 className="text-xl font-bold mb-1">Enter a new email address</h3>
        <p className="text-center mb-5 text-gray-600">Enter your new email address and your existing password and click done to confirm it.</p>
        <div className="flex flex-col gap-4 ">
          <div>
            <p className="font-semibold mb-1">Email</p>
            <input
              value={newEmail}
              onChange={(e) => setNewEmail(e.target.value)}
              className="border w-80 bg-gray-100 border-gray-200 px-2 py-2 rounded-md"
            />
          </div>
          <div>
            <p className="font-semibold mb-1">Password</p>
            <input
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="border w-80 bg-gray-100 border-gray-200 px-2 rounded-md py-2"
              type="password"
            />
          </div>
          <button className="bg-blue-500 hover:bg-blue-600 w-fit text-white px-6 rounded-md py-2 mt-3 self-end"
            onClick={() => handleSubmit()}>
            Done
          </button>
        </div>
      </ModalContainer>
      <AlertDialog visible={changeFailed} message={alertMessage} color="text-red-500">
      </AlertDialog>
    </>
  )
}
