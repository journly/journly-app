import { Avatar, Box } from "@mui/material";
import { useAuth } from "../../providers/AuthProvider";
import { useEffect, useRef, useState } from "react";
import { Pencil as EditIcon, Undo } from "lucide-react";
import { SettingsContainer } from "../../components/settings/SettingsContainer";
import { ChangeEmailModal } from "../../components/settings/ChangeEmailModal";
import { ChangePasswordModal } from "../../components/settings/ChangePasswordModal";
import { DeleteAccountModal } from "../../components/settings/DeleteAccountModal";
import { AlertDialog } from "../../components/settings/AlertDialog";

export default function MyAccountPage() {
  const { getUser, refreshUser } = useAuth();
  const [username, setUsername] = useState(getUser()?.username ?? "Undefined");
  const [email, setEmail] = useState(getUser()?.email ?? "Undefined");
  const [showEmailModal, setShowEmailModal] = useState(false);
  const [showPasswordModal, setShowPasswordModal] = useState(false);
  const [showDeleteAccountModal, setShowDeleteAccountModal] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const alertTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [alertMessage, setAlertMessage] = useState("");

  const onChangeEmailSuccess = (newEmail: string) => {
    setEmail(newEmail);
    setAlertMessage("Email successfully updated!");
    setShowAlert(true);
    setShowEmailModal(false);
    refreshUser();
  }

  const onChangeUsernameSuccess = () => {

  }

  useEffect(() => {
    if (showAlert) {
      if (alertTimeoutRef.current) {
        clearTimeout(alertTimeoutRef.current);
        alertTimeoutRef.current = null;
      }

      alertTimeoutRef.current = setTimeout(() => {
        setShowAlert(false);
      }, 3000)
    }

    return () => {
      if (alertTimeoutRef.current) {
        clearTimeout(alertTimeoutRef.current)
      }
    }
  }, [showAlert])

  return (
    <>
      <SettingsContainer>
        <Box className="flex flex-col gap-7">
          <h3 className="border-b border-gray-200 text-gray-500 font-semibold text-lg">Account</h3>
          <Box className="flex flex-row gap-6 items-center">
            <Box className="relative">
              <Box className="absolute z-10 opacity-0 hover:opacity-30 bg-gray-200 w-full h-full rounded-full flex justify-center items-center cursor-pointer" >
                <EditIcon />
              </Box>
              <Avatar sx={{ width: 60, height: 60 }}>
                {getUser()?.avatar ?
                  <img src={getUser()?.avatar ?? ""} />
                  :
                  getUser()?.username.charAt(0).toUpperCase() || 'JD'
                }
              </Avatar>
            </Box>
            <Box>
              <p className="font-semibold mb-1">
                Username
              </p>
              <Box className="flex">
                <input value={username} onChange={(e) => setUsername(e.target.value)} className="border bg-gray-100 border-gray-200 px-2 rounded-md" />
                {
                  getUser()?.username != username &&
                  <>
                    <button className="px-2 bg-green-500 rounded-md text-white drop-shadow-md mx-3">
                      Save
                    </button>
                    <button
                      className=" bg-gray-500 rounded-md drop-shadow-md px-0.5"
                      onClick={() => setUsername(getUser()?.username ?? "Undefined")}
                    >
                      <Undo color="white" size={22} />
                    </button>
                  </>
                }
              </Box>
            </Box>
          </Box>
          <Box className="flex justify-between items-center">
            <Box>
              <p className="font-semibold ">
                Email
              </p>
              <p>
                {email}
              </p>
            </Box>
            <button
              className="bg-blue-500 px-4 py-1 rounded-md text-white hover:bg-blue-600"
              onClick={() => { setShowEmailModal(true); console.log("hello", showEmailModal) }}
            >
              Change Email
            </button>
          </Box>
        </Box>
        <Box className="flex flex-col gap-7">
          <h3 className="mt-10 border-b border-gray-200 text-gray-500 font-semibold text-lg">Password and Authentication</h3>
          <button
            className="bg-blue-500 px-4 py-1 rounded-md text-white hover:bg-blue-600 w-fit"
            onClick={() => setShowPasswordModal(true)}
          >Change Password</button>
          <Box>
            <h4 className="font-semibold text-gray-500 ">Account Deletion</h4>
            <p className="text-gray-400 mb-2 leading-5">Deleting your account means all data pertaining to this account will be deleted forever. This includes all trips and journals that this account is the sole contributor to. Trips and journals with other contributors will still remain accessible.</p>

            <button className="bg-red-500 text-white px-4 py-1 rounded-md" onClick={() => setShowDeleteAccountModal(true)}>Delete Account</button>
          </Box>
        </Box>
      </SettingsContainer>
      <ChangeEmailModal onClose={() => setShowEmailModal(false)} isOpen={showEmailModal} onUpdateSuccess={onChangeEmailSuccess} />
      <ChangePasswordModal onClose={() => setShowPasswordModal(false)} isOpen={showPasswordModal} />
      <DeleteAccountModal onClose={() => setShowDeleteAccountModal(false)} isOpen={showDeleteAccountModal} />
      <AlertDialog visible={showAlert} message={alertMessage} color="text-green-500" />
    </>
  )
}
