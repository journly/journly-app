import { X } from "lucide-react";
import { ReactNode } from "react";
import { createPortal } from "react-dom";

interface ModalContainerProps {
  children: ReactNode;
  onClose: () => void;
}

export const ModalContainer: React.FC<ModalContainerProps> = ({ children, onClose }) => {
  const root = document.getElementById('root');

  if (!root) return null;

  return (
    createPortal(
      <div className="fixed inset-0 z-50 bg-black bg-opacity-40 flex justify-center items-center w-full h-full ">
        <div className="bg-white drop-shadow-md px-10 py-5 rounded-lg relative flex flex-col items-center max-w-[460px]">
          {children}
          <div
            className="absolute right-2 top-2 cursor-pointer"
            onClick={onClose}
          >
            <X />
          </div>
        </div>
      </div>, root
    )
  )
}
