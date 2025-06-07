import { Avatar, AvatarGroup, Box, Button, Typography } from "@mui/material";
import { useTrip } from "../providers/TripProvider";
import { ArrowLeft, CalendarDaysIcon, MapPinIcon, Users } from "lucide-react";
import { NavLink } from "react-router-dom";
import { useState } from "react";
import DialogWrapper from "./generic/DialogWrapper";

 
 interface TripBannerProps {
    
    
 }
 
 
 
const TripBanner: React.FC<TripBannerProps> = () => {
    const { trip } = useTrip();

    const [usersDialogOpen, setUsersDialogOpen] = useState<boolean>(false);

    const startDate = trip?.travelDates.startDate;
    const endDate = trip?.travelDates.endDate;



   return (
        <Box className="bg-cover bg-center overflow-hidden bg-slate-600" style={{ backgroundImage: 'url(/images/trip-banner.jpg)' }}>
            <div className='pb-5 px-4 pt-14 lg:pt-16 2xl:pt-24 max-w-screen-2xl mx-auto'>
                <div className="flex items-center justify-between mb-4">
                    <Box className="flex-col items-center gap-2 mx-5 text-white">
                        <NavLink to={"/"}>
                            <Button variant="text" sx={{color: "lightgray"}} startIcon={<ArrowLeft size={20}/>} disableFocusRipple>
                                Back to Dashboard
                            </Button>   
                        </NavLink>
                        <Typography variant="h6" marginTop={2} marginLeft={1}>
                            Japan Adventure
                        </Typography>
                        <Box className="flex items-center gap-5 mt-2 lg:mt-4">
                            <Box className="flex items-center gap-2">
                                <CalendarDaysIcon className="h-5 w-5" />
                                <p>
                                    {startDate.monthShort} {startDate.day}-{endDate.day}, {endDate.year}
                                </p>
                            </Box>
                            <Box className="flex items-center gap-2">
                                <MapPinIcon className="h-5 w-5" />
                                <p>{trip?.locations?.join(' → ')}</p>
                            </Box>
                        </Box>
                    </Box>
                    <Box className="flex items-center gap-2 mt-5 lg:mt-7">
                        <AvatarGroup max={4} sx={{ '& .MuiAvatar-root': { width: 28, height: 28, fontSize: 14 } }} spacing="medium" >
                            {
                                trip?.users?.map((user) => (
                                    <Avatar key={user.username} alt={user.initials} />
                                ))
                            }
                            <Avatar alt="A" src="/avatars/user1.jpg"/>
                            <Avatar alt="B" src="/avatars/user1.jpg"/>
                            <Avatar alt="C" src="/avatars/user1.jpg"/>
                            <Avatar alt="D" src="/avatars/user1.jpg"/>
                            <Avatar alt="E" src="/avatars/user1.jpg"/>
                        </AvatarGroup>
                        <Box className="flex items-center justify-center p-2 rounded-full cursor-pointer hover:bg-gray-400 transition-colors" onClick={() => setUsersDialogOpen(true)}>
                            <Users className="h-5 w-5" color="white" />
                        </Box>
                    </Box>
                </div>
            </div>
            <DialogWrapper
                open={usersDialogOpen}
                onClose={() => setUsersDialogOpen(false)}
                title="Trip Travelers"
            >
                <Box className="p-4">
                    <Typography variant="body1" >
                        Here are the travelers for this trip:
                    </Typography>
                    <Box className="flex flex-col gap-2 p-0">
                        {trip?.users?.map((user) => (
                            <Box key={user.username} className="flex items-center gap-3 p-2 bg-gray-100 rounded-md">
                                <Avatar alt={user.initials} src={"/avatars/default.jpg"} />
                                <Typography variant="body2">{user.fullName || user.username}</Typography>
                            </Box>
                        ))}
                    </Box>
                </Box>
            </DialogWrapper>
        </Box>
   );
 }

export default TripBanner;