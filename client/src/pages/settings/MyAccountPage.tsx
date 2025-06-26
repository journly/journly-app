import { Avatar, Box } from "@mui/material";
import { useAuth } from "../../providers/AuthProvider";
import { useState } from "react";
import { Pencil as EditIcon } from "lucide-react";

export default function MyAccountPage() {
  const { getUser } = useAuth();
  const [username, setUsername] = useState(getUser()?.username ?? "Undefined");

  return (
    <Box className="flex w-full justify-center">
      <Box className="max-w-2xl w-full bg-white mx-7 my-10 px-10 py-5 flex-col items-center rounded-xl drop-shadow-sm">
        <Box className="flex flex-col gap-7">
          <h3 className="border-b border-gray-200 text-gray-500 font-semibold text-lg">Account</h3>
          <Box className="flex flex-row gap-6 items-center">
            <Box className="relative">
              <Box className="absolute z-10 opacity-0 hover:opacity-30 bg-gray-200 w-full h-full rounded-full flex justify-center items-center cursor-pointer" >
                <EditIcon />
              </Box>
              <Avatar sx={{ width: 60, height: 60 }} >
                {getUser()?.avatar ?
                  <img src={getUser()?.avatar ?? ""} />
                  :
                  getUser()?.username.charAt(0).toUpperCase() || 'JD'
                }
              </Avatar>
            </Box>
            <Box>
              <p className="font-semibold mb-1 ">
                Username
              </p>
              <input value={username} onChange={(e) => setUsername(e.target.value)} className="border bg-gray-100 border-gray-200 px-2 rounded-md" />
            </Box>
          </Box>
          <Box className="flex justify-between items-center">
            <Box>
              <p className="font-semibold ">
                Email
              </p>
              <p>
                {getUser()?.email}
              </p>
            </Box>
            <button className="bg-blue-500 px-4 py-1 rounded-md text-white hover:bg-blue-600">
              Change Email
            </button>
          </Box>

        </Box>

        <Box className="flex flex-col gap-7">
          <h3 className="mt-10 border-b border-gray-200 text-gray-500 font-semibold text-lg">Password and Authentication</h3>
          <button className="bg-blue-500 px-4 py-1 rounded-md text-white hover:bg-blue-600 w-fit">Change Password</button>
          <Box>
            <h4 className="font-semibold text-gray-500 ">Account Deletion</h4>
            <p className="text-gray-400 mb-2 leading-5">Deleting your account means all data pertaining to this account will be deleted forever. This includes all trips and journals that this account is the sole contributor to. Trips and journals with other contributors will still remain accessible.</p>

            <button className="bg-red-500 text-white px-4 py-1 rounded-md">Delete Account</button>
          </Box>
        </Box>
      </Box>
    </Box>
  )
}
