import { ReactElement, useRef, useState } from "react";
import {
  Box,
  Button,
} from "@mui/material";
import {
  User as UserIcon,
  Bell as BellIcon,
  ArrowLeft as ArrowLeftIcon,
  LogOut as LogOutIcon,
  Settings2Icon
} from "lucide-react";
import { useAuth } from "../../providers/AuthProvider";
import { MenuItem } from "../menu/MenuItem";
import { LogoutConfirmModal } from "../LogoutConfirmModal";

interface MenuItem {
  key: string;
  label: string;
  icon: ReactElement,
  content?: ReactElement,
  subItems?: MenuItem[]
}


export default function SettingsBar() {
  const { logout } = useAuth();
  const [showLogoutModal, setShowLogoutModal] = useState(false);
  const previousRouteRef = useRef(window.history.length - 1);

  const handleLogout = async () => {
    await logout();
  }

  return (
    <>
      <Box className="w-64 bg-white border-r border-gray-200 flex flex-col p-4 overflow-y-auto">
        <h1 className="text-2xl font-semibold text-blue-600 mb-6 ">Settings</h1>
        <Box className="flex-1">
          <MenuItem icon={<UserIcon />} label="My Account" link="account" />
          <MenuItem icon={<BellIcon />} label="Notifications" link="notifications" />
          <MenuItem icon={<Settings2Icon />} label="Preferences" link="preferences" />
          <MenuItem
            icon={<LogOutIcon />}
            label="Logout"
            onClick={() => setShowLogoutModal(true)}
            iconColor="text-red-500"
            textColor="#ef4444"
          />
        </Box>


        <Box className="p-3 border-t border-gray-200">
          <Button
            startIcon={<ArrowLeftIcon />}
            onClick={() => window.history.go(previousRouteRef.current - window.history.length)}
            className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm"
          >
            Back
          </Button>
        </Box>
      </Box>
      <LogoutConfirmModal
        isOpen={showLogoutModal}
        onCancel={() => setShowLogoutModal(false)}
        onConfirm={handleLogout}
      />
    </>
  );
}
