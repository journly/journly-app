import { Box, Dialog } from "@mui/material"
import { useNavigate } from "react-router-dom";
import { useAuth } from "../../providers/AuthProvider";

interface OnDeleteAccountModalProps {
  isOpen: boolean;
}

export const OnDeleteAccountModal: React.FC<OnDeleteAccountModalProps> = ({ isOpen }) => {
  const navigate = useNavigate();

  const { logout } = useAuth();

  const handleClose = async () => {
    await logout();

    navigate("/");
  }

  return (
    <Dialog
      open={isOpen}
    >
      <Box className="px-5 py-5 rounded-md flex flex-col items-center gap-5">
        <h3 className="text-lg font-semibold">Your account was deleted.</h3>
        <p>We're sorry to see you go!</p>
        <button className="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-md text-white w-fit self-end"
          onClick={handleClose}
        >
          Done
        </button>
      </Box>

    </Dialog>
  )

}
