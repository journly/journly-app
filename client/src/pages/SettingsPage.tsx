import { useState } from "react";
import {
  TextField,
  Switch,
  FormControlLabel,
  Box,
  Button,
} from "@mui/material";
import {
  Cog as SettingsIcon,
  User as UserIcon,
  Lock as LockIcon,
  Bell as BellIcon,
  Moon as MoonIcon,
  Key as KeyIcon,
  ArrowLeft as ArrowLeftIcon,
  PencilIcon,
  MailCheckIcon,
  MailIcon,
} from "lucide-react";
import { MenuItem } from "../components/menu/MenuItem";

const MENU_STRUCTURE = [
  {
    key: "profile",
    label: "Profile",
    icon: <UserIcon />,
    content: (
      <Box component="form" noValidate autoComplete="off" className="max-w-md space-y-6">
        <TextField fullWidth label="Full Name" defaultValue="John Doe" />
        <TextField fullWidth label="Email Address" defaultValue="john@example.com" />
      </Box>
    ),
    subItems: [
      {
        key: "editProfile",
        label: "Edit Profile",
        icon: <PencilIcon />,
        content: (
          <Box component="form" noValidate autoComplete="off" className="max-w-md space-y-6">
            <TextField fullWidth label="Full Name" defaultValue="John Doe" />
          </Box>
        ),  
      },
      {
        key: "invites",
        label: "Trip Invites",
        icon: <MailIcon />,
        content: (
          <Box className="max-w-md space-y-6">
            <p className="text-gray-600">You have no pending invites.</p>
          </Box>
        ),
      }
    ],
  },
  {
    key: "account",
    label: "Account",
    icon: <SettingsIcon />,
    subItems: [
      {
        key: "notifications",
        label: "Notifications",
        icon: <BellIcon />,
        content: (
          <FormControlLabel
            control={<Switch defaultChecked color="primary" />}
            label="Enable notifications"
          />
        ),
      },
    ],
  },
  {
    key: "security",
    label: "Security",
    icon: <LockIcon />,
    subItems: [
      {
        key: "darkMode",
        label: "Dark Mode",
        icon: <MoonIcon />,
        content: (
          <FormControlLabel
            control={<Switch defaultChecked color="primary" />}
            label="Enable dark mode"
          />
        ),
      },
      {
        key: "changePassword",
        label: "Change Password",
        icon: <KeyIcon />,
        content: <TextField fullWidth label="New Password" type="password" />,
      },
    ],
  },
];

export default function SettingsPage() {
  // Start with first main section or first subItem if available
  const firstKey =
    MENU_STRUCTURE[0].subItems?.[0]?.key ?? MENU_STRUCTURE[0].key;

  const [activeKey, setActiveKey] = useState(firstKey);

  // Find active content from menu structure
  let activeContent = null;
  for (const section of MENU_STRUCTURE) {
    if (section.key === activeKey) {
      activeContent = section.content;
      break;
    }
    if (section.subItems) {
      const sub = section.subItems.find((item) => item.key === activeKey);
      if (sub) {
        activeContent = sub.content;
        break;
      }
    }
  }

  return (
    <div className="flex h-screen bg-gray-50 relative">
        
      <nav className="w-64 bg-white border-r border-gray-200 flex flex-col p-4 overflow-y-auto">
        <h1 className="text-2xl font-semibold text-blue-600 mb-6">Settings</h1>
        
        <div className="flex-1">
            {MENU_STRUCTURE.map((section) => (
                <div key={section.key} className="border-b border-gray-200 mb-1">
                <MenuItem
                    icon={section.icon}
                    label={section.label}
                />
                {section.subItems && (
                    <div className="ml-6 mt-2 flex flex-col ">
                    {section.subItems.map((sub) => (
                        <MenuItem
                        key={sub.key}
                        icon={sub.icon}
                        label={sub.label}
                        />
                    ))}
                    </div>
                )}
                </div>
            ))}
        </div>


        <div className="p-3 border-t border-gray-200">
            <Button
                startIcon={<ArrowLeftIcon />}
                onClick={() => window.history.back()}
                className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm"
            >
                Back
            </Button>
        </div>
      </nav>

    </div>
  );
}