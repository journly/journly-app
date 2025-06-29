import React, { useState } from "react";
import { ModalContainer } from "./ModalContainer";
import { AlertDialog } from "./AlertDialog";
import { useUser } from "../../providers/UserProvider";

interface ChangeEmailModalProps {
  onClose: () => void;
  onUpdateSuccess: (email: string) => void;
}

export const ChangeEmailModal: React.FC<ChangeEmailModalProps> = ({ onClose, onUpdateSuccess }) => {
  const { updateEmail, validateUserPassword } = useUser();
  const [newEmail, setNewEmail] = useState("");
  const [highlightNewEmail, setHighlightNewEmail] = useState(false);
  const [password, setPassword] = useState("");
  const [highlightPassword, setHighlightPassword] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");

  const root = document.getElementById('root');

  if (!root) return null;

  const handleSubmit = async () => {
    if (!newEmail.length) {
      setAlertMessage("Email cannot be empty.");
      setShowAlert(true);
      setHighlightNewEmail(true);
      return
    }

    if (!password.length) {
      setAlertMessage("Enter your password first.");
      setShowAlert(true);
      setHighlightPassword(true);
      return
    }

    try {
      let res = await validateUserPassword(password);

      if (res) {
        try {
          let successful = await updateEmail(newEmail);

          if (!successful) throw new Error();

          onUpdateSuccess(newEmail);
        } catch {
          setShowAlert(true);
          setAlertMessage("Invalid email.");
        }
      } else {
        setShowAlert(true);
        setHighlightPassword(true);
        setAlertMessage("Password was incorrect.")
      }

    } catch {
      setShowAlert(true);
    }
  }

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
              onChange={(e) => { setNewEmail(e.target.value); if (highlightNewEmail) setHighlightNewEmail(false) }}
              className={"border w-80 bg-gray-100 px-2 py-2 rounded-md " + (highlightNewEmail ? "border-red-500" : "border-gray-200")}
            />
          </div>
          <div>
            <p className="font-semibold mb-1">Password</p>
            <input
              value={password}
              onChange={(e) => { setPassword(e.target.value); if (highlightPassword) setHighlightPassword(false) }}
              className={"border w-80 bg-gray-100 px-2 rounded-md py-2 " + (highlightPassword ? "border-red-500" : "border-gray-200")}
              type="password"
            />
          </div>
          <button className="bg-blue-500 hover:bg-blue-600 w-fit text-white px-6 rounded-md py-2 mt-3 self-end"
            onClick={() => handleSubmit()}>
            Done
          </button>
        </div>
      </ModalContainer>
      <AlertDialog visible={showAlert} message={alertMessage} color="text-red-500" toggleVisibility={() => setShowAlert(!showAlert)} />
    </>
  )
}
