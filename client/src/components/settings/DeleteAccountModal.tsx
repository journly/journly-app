import { useState } from "react";
import { ModalContainer } from "./ModalContainer";

interface DeleteAccountModalProps {
  onClose: () => void;
}

export const DeleteAccountModal: React.FC<DeleteAccountModalProps> = ({ onClose }) => {
  const [password, setPassword] = useState("");

  return (
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
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>

      <div className="flex gap-10 mt-5">
        <button className="bg-blue-500 px-4 py-2 rounded-md text-white" onClick={onClose}>
          Cancel
        </button>
        <button
          className="bg-red-500 hover:bg-red-600 px-4 py-2 rounded-md text-white disabled:bg-red-800"
          disabled={password.length == 0}
        >
          Confirm
        </button>
      </div>
    </ModalContainer>
  )
}
