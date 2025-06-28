import { Box } from "@mui/material";
import React, { ReactNode } from "react";

export const SettingsContainer: React.FC<{ children: ReactNode }> = ({ children }) => {
  return (
    <Box className="flex w-full justify-center">
      <Box className="max-w-2xl w-full bg-white mx-7 my-10 px-10 py-5 flex-col items-center rounded-xl drop-shadow-sm">
        {children}
      </Box>
    </Box>
  )
}
