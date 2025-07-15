import { Avatar, Box } from "@mui/material";
import { useState } from "react";
import { Pencil as EditIcon, Undo } from "lucide-react";
import { SettingsContainer } from "../../components/settings/SettingsContainer";
import { ChangeEmailModal } from "../../components/settings/ChangeEmailModal";
import { ChangePasswordModal } from "../../components/settings/ChangePasswordModal";
import { DeleteAccountModal } from "../../components/settings/DeleteAccountModal";
import { AlertDialog } from "../../components/settings/AlertDialog";
import { useUser } from "../../providers/UserProvider";
import { ProfilePictureModal } from "../../components/settings/ProfilePictureModal";
import { OnDeleteAccountModal } from "../../components/settings/OnDeleteAccountModal";

const errorAlertColor = "text-red-500";
const successAlertColor = "text-green-500";

export default function MyAccountPage() {
  const { user, updateUsername } = useUser();
  const [username, setUsername] = useState(user?.username ?? "Undefined");
  const [email, setEmail] = useState(user?.email ?? "Undefined");
  const [showEmailModal, setShowEmailModal] = useState(false);
  const [showPasswordModal, setShowPasswordModal] = useState(false);
  const [showDeleteAccountModal, setShowDeleteAccountModal] = useState(false);
  const [showAlert, setShowAlert] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");
  const [alertMessageColor, setAlertMessageColor] = useState("")
  const [showProfilePictureModal, setShowProfilePictureModal] = useState(false);
  const [accountDeleted, setAccountDeleted] = useState(false);

  const triggerAlert = (message: string, success: boolean) => {
    setShowAlert(true);
    setAlertMessage(message);
    setAlertMessageColor(success ? successAlertColor : errorAlertColor);
  }

  const onChangeEmail = (newEmail: string) => {
    setEmail(newEmail);
    triggerAlert("Email successfully updated!", true);

    setShowEmailModal(false);
  }

  const onChangeUsername = async (newUsername: string) => {
    if (!newUsername.length) {
      setUsername(user?.username ?? "Undefined");
      triggerAlert("Invalid username.", false);
      return
    }

    try {
      let successful = await updateUsername(username);

      if (!successful) throw new Error();

      triggerAlert("Username successfully updated!", true);
    } catch {
      setUsername(user?.username ?? "Undefined");
      triggerAlert("Invalid username.", false);
    }
  }

  const onChangePassword = () => {
    triggerAlert("Password successfully updated!", true);
    setShowPasswordModal(false);
  }

  const onChangeProfilePicture = () => {
    triggerAlert("Profile picture successfully updated!", true);
    setShowProfilePictureModal(false);
  }

  const onDeleteAccount = () => {
    setShowDeleteAccountModal(false);
    setAccountDeleted(true);
  }

  return (
    <>
      <SettingsContainer>
        <Box className="flex flex-col gap-7">
          <h3 className="border-b border-gray-200 text-gray-500 font-semibold text-lg">Account</h3>
          <Box className="flex flex-row gap-6 items-center">
            <Box className="relative">
              <Box
                className="absolute z-10 opacity-0 hover:opacity-30 bg-gray-200 w-full h-full rounded-full flex justify-center items-center cursor-pointer"
                onClick={() => setShowProfilePictureModal(true)}
              >
                <EditIcon />
              </Box>
              <Avatar sx={{ width: 60, height: 60 }}>
                {user?.avatar ?
                  <img src={user?.avatar ?? ""} />
                  :
                  user?.username.charAt(0).toUpperCase() || 'JD'
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
                  user?.username != username &&
                  <>
                    <button className="px-2 bg-green-500 rounded-md text-white drop-shadow-md mx-3"
                      onClick={() => onChangeUsername(username)}
                    >
                      Save
                    </button>
                    <button
                      className=" bg-gray-500 rounded-md drop-shadow-md px-0.5"
                      onClick={() => setUsername(user?.username ?? "Undefined")}
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
              onClick={() => setShowEmailModal(true)}
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
      {showProfilePictureModal && <ProfilePictureModal onClose={() => setShowProfilePictureModal(false)} onUpdateSuccess={onChangeProfilePicture} />}
      {showEmailModal && <ChangeEmailModal onClose={() => setShowEmailModal(false)} onUpdateSuccess={onChangeEmail} />}
      {showPasswordModal && <ChangePasswordModal onClose={() => setShowPasswordModal(false)} onUpdateSuccess={onChangePassword} />}
      {showDeleteAccountModal && <DeleteAccountModal onClose={() => setShowDeleteAccountModal(false)} onUpdateSuccess={onDeleteAccount} />}
      <AlertDialog visible={showAlert} message={alertMessage} color={alertMessageColor} toggleVisibility={() => setShowAlert(!showAlert)} />
      <OnDeleteAccountModal isOpen={accountDeleted} />
    </>
  )
}
