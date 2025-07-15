import { useState } from "react";
import { ModalContainer } from "./ModalContainer";
import { useUser } from "../../providers/UserProvider";
import { AlertDialog } from "./AlertDialog";

interface DeleteAccountModalProps {
  onClose: () => void;
  onUpdateSuccess: () => void;
}

export const DeleteAccountModal: React.FC<DeleteAccountModalProps> = ({ onClose, onUpdateSuccess }) => {
  const { user, deleteUser } = useUser();
  const [email, setEmail] = useState("");
  const [showAlert, setShowAlert] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");

  const onSubmitHandler = async () => {
    if (!user) return;

    if (user.email == email) {
      let success = await deleteUser();

      if (!success) {
        setShowAlert(true);
        setAlertMessage("Could not delete account.");
      } else {
        onUpdateSuccess();
      }
    }
    else {
      setAlertMessage("Email is incorrect.");
      setShowAlert(true);
    }
  }

  return (
    <>
      <ModalContainer onClose={onClose}>
        <h3 className="font-bold text-xl text-center">
          Are you sure you want to delete your account?
        </h3>
        <div>
          <p className="text-center text-gray-600 mt-3">
            If you are sure you want to delete your account type out your current email
          </p>
          <input
            className="px-2 py-2 mt-3 w-full rounded-lg bg-gray-100 border-gray-200"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </div>

        <div className="flex gap-10 mt-5">
          <button className="bg-blue-500 px-4 py-2 rounded-md text-white" onClick={onClose}>
            Cancel
          </button>
          <button
            className="bg-red-500 hover:bg-red-600 px-4 py-2 rounded-md text-white disabled:bg-red-800"
            disabled={email.length == 0}
            onClick={onSubmitHandler}
          >
            Confirm
          </button>
        </div>
      </ModalContainer>
      <AlertDialog visible={showAlert} color="text-red-500" message={alertMessage} toggleVisibility={() => setShowAlert(!showAlert)} />
    </>
  )
}
