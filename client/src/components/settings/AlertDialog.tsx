import React from "react"

interface AlertDialogProps {
  message: string;
  color?: string;
  visible: boolean;
}

export const AlertDialog: React.FC<AlertDialogProps> = ({ visible, color, message }) => {
  return (
    <div className={"fixed drop-shadow-lg inset-0 z-[51] top-[85%] h-fit left-1/2 -translate-x-1/2 w-fit bg-white px-10 py-5 rounded-xl transition-all duration-300 ease-in-out "
      + (visible ? "translate-y-0 opacity-100" : "translate-y-full opacity-0")
    }>
      <p className={color ?? ""}>
        {message}
      </p>
    </div>
  )
}
