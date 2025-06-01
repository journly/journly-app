import { MapIcon } from "lucide-react";
import { MenuItem } from "./MenuItem";
import { Box } from "@mui/material";

interface TripItemProps {
  name: string;
  key?: string;
  date: string;
}


export function TripItem({
  name,
  key = name,
  date
}: TripItemProps) {

  return (
    <MenuItem 
      icon={
        <Box className="w-6 h-6 rounded bg-gray-100 flex items-center justify-center">
          <MapIcon className="h-3 w-3 text-gray-500" />
        </Box>
        } 
      label={name} 
      smallLabel={date}
      link={`/trip/${key}`}
      />
  )
}