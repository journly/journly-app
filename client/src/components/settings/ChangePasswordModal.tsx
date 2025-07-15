import { useState } from "react";
import { ModalContainer } from "./ModalContainer"
import { AlertDialog } from "./AlertDialog";
import { useUser } from "../../providers/UserProvider";

interface ChangePasswordModalProps {
  onClose: () => void;
  onUpdateSuccess: () => void;
}

export const ChangePasswordModal: React.FC<ChangePasswordModalProps> = ({ onClose, onUpdateSuccess }) => {
  const { validateUserPassword, updatePassword } = useUser();
  const [currentPassword, setCurrentPassword] = useState("");
  const [highlightPasswordInput, setHighlightPasswordInput] = useState(false);
  const [newPassword, setNewPassword] = useState("");
  const [highlightNewPasswordInput, setHighlightNewPasswordInput] = useState(false);
  const [confirmNewPassword, setConfirmNewPassword] = useState("");
  const [highlightConfirmNewPasswordInput, setHighlightConfirmNewPasswordInput] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");

  const onSubmitHandler = async () => {
    if (!currentPassword.length || !newPassword.length || !confirmNewPassword.length) {
      if (!currentPassword.length) setHighlightPasswordInput(true);
      if (!newPassword.length) setHighlightNewPasswordInput(true);
      if (!confirmNewPassword.length) setHighlightConfirmNewPasswordInput(true);

      setAlertMessage("Password cannot be empty.");
      setShowAlert(true);

      return
    }

    if (newPassword !== confirmNewPassword) {
      setAlertMessage("Passwords are not matching.");
      setShowAlert(true);

      setHighlightNewPasswordInput(true);
      setHighlightConfirmNewPasswordInput(true);
      return
    }

    if (currentPassword.length && newPassword.length && confirmNewPassword.length) {
      try {
        let valid = await validateUserPassword(currentPassword);

        if (!valid) {
          setAlertMessage("Incorrect password.");
          setShowAlert(true);
          return
        }

        await updatePassword(currentPassword, newPassword);

        onUpdateSuccess();
      } catch {
        setAlertMessage("Failed to update password.");
        setShowAlert(true);
      }
    }
  }

  return (
    <>
      <ModalContainer onClose={onClose}>
        <h3 className="font-bold text-xl mb-1">Change password</h3>
        <p className="text-center mb-5 text-gray-600">Enter your current password and the new password you want to change to.</p>
        <div className="flex flex-col gap-4 ">
          <div>
            <p className="font-semibold mb-1">Current Password</p>
            <input
              value={currentPassword}
              onChange={(e) => {
                setCurrentPassword(e.target.value);
                if (highlightPasswordInput) setHighlightPasswordInput(false);
              }}
              className={"border w-80 bg-gray-100 px-2 py-2 rounded-md " + (highlightPasswordInput ? "border-red-500" : "border-gray-200")}
              type="password"
            />
          </div>
          <div>
            <p className="font-semibold mb-1">New Password</p>
            <input
              value={newPassword}
              onChange={(e) => {
                setNewPassword(e.target.value);
                if (highlightNewPasswordInput) setHighlightNewPasswordInput(false);
              }}
              className={"border w-80 bg-gray-100 px-2 py-2 rounded-md " + (highlightNewPasswordInput ? "border-red-500" : "border-gray-200")}
              type="password"
            />
          </div>
          <div>
            <p className="font-semibold mb-1">Confirm New Password</p>
            <input
              value={confirmNewPassword}
              onChange={(e) => {
                setConfirmNewPassword(e.target.value);
                if (highlightConfirmNewPasswordInput) setHighlightConfirmNewPasswordInput(false);
              }}
              className={"border w-80 bg-gray-100 px-2 py-2 rounded-md " + (highlightConfirmNewPasswordInput ? "border-red-500" : "border-gray-200")}
              type="password"
            />
          </div>
          <button className="bg-blue-500 hover:bg-blue-600 w-fit text-white px-6 rounded-md py-2 mt-3 self-end"
            onClick={() => onSubmitHandler()}
          >
            Done
          </button>

        </div>
      </ModalContainer>
      <AlertDialog visible={showAlert} toggleVisibility={() => setShowAlert(!showAlert)} color="text-red-500" message={alertMessage} />
    </>
  )
}
