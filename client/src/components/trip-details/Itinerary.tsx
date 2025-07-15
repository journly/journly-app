import { Avatar, Box, Button, List, Typography } from "@mui/material";
import Section from "../generic/Section.tsx";
import { ClockIcon, Dot, ListTodoIcon, MapIcon, PlusIcon, SparklesIcon } from "lucide-react";
import { useTrip } from "../../providers/TripProvider.tsx";
import TravelAssistance from "../generic/TravelAssistance.tsx";
import { useState } from "react";
import DialogWrapper from "../generic/DialogWrapper.tsx";

const Itinerary = () =>{
    const { trip } = useTrip();
    const [daysDialogOpen, setDaysDialogOpen] = useState<boolean>(false);
    const itinerary: any[] = [
        { title: "Day 1", activities: ["Arrive at destination", "Check into hotel", "Explore local area"] },
        { title: "Day 2", activities: ["Visit museum", "Lunch at local restaurant", "Evening city tour"] },
    ];
    const startDate = trip?.travelDates.startDate;
    const endDate = trip?.travelDates.endDate;
    const daysRemaining = endDate.diff(startDate, "days").days

    const handleDaysDialog = (day: any) => {
        setDaysDialogOpen(true);
    }

    return (
        <>
        <Box className="grid grid-cols-5 md:grid-cols-7 gap-4 ">
            <Box className="grid grid-rows-5 col-span-4 md:col-span-5">
                <Box className="row-span-1 flex items-center justify-between gap-2 pb-2">
                    <Box className="flex items-center gap-2 pb-2 pl-2">
                        <ClockIcon className="text-gray-600" size={18} />
                        <Typography variant="caption" fontSize={14} className="text-gray-600">
                            {daysRemaining} days remaining
                        </Typography>
                    </Box>
                    <Button
                        variant="outlined"
                        size="small"
                        sx={{ bgcolor: 'primary.main', color: 'white', '&:hover': { bgcolor: 'primary.dark' } }}
                        onClick={() => {
                            // Handle add task action
                        }}
                    >
                        <MapIcon className="mr-1" size={16} color="white"/>
                        View on Map
                    </Button>
                </Box>
                <Section additionalStyle="row-span-4 px-0 pt-2" >
                    <Box className="flex items-center justify-between gap-2 border-b px-5 p-2 pb-4">
                        <Box className="flex justify-center pl-2">
                            <Typography variant="h5" fontWeight={500} fontSize={18} className="text-gray-900">
                                Daily Itinerary
                            </Typography>
                        </Box>
                        <Box className="flex justify-center">
                            <Button
                                variant="text"
                                size="small"
                                sx={{padding: '0', color: 'primary.main', fontWeight: 500, textTransform: 'none'}}
                                onClick={() => {
                                    // Handle add task action
                                }}
                            >
                                <PlusIcon size={16} />
                                Add Day
                            </Button>
                        </Box>
                    </Box>
                    <Box className="flex-1 gap-2 mx-4">
                        <List className="mt-4">                
                        {itinerary?.map((day, index) => (
                            <Box key={index} className="p-4 bg-white border rounded-lg gap-2 mb-2">
                                <Box className="flex items-center justify-between" >
                                    <Box className="gap-2">
                                        <Typography variant="body1" fontWeight={500} className="text-gray-800 mb-2">
                                            Day {index + 1}
                                        </Typography>
                                        <Typography variant="caption" className="flex text-gray-400">
                                            Nov 15
                                            <Dot className="mx-1 text-gray-400" size={20} />
                                            Tokyo
                                        </Typography>
                                    </Box>
                                    <Box 
                                        className="flex items-center justify-center p-2 rounded-full cursor-pointer hover:bg-gray-200 transition-colors" 
                                        onClick={() => handleDaysDialog(day)}
                                    >
                                        <ListTodoIcon size={16} color="grey" />
                                    </Box>
                                </Box>
                                <List>
                                        
                                </List>
                            </Box>
                        ))}
                        </List>
                    </Box>
                </Section>
            </Box>
            <Section additionalStyle="col-span-1 md:col-span-2">
                <TravelAssistance
                    iconColor="text-blue-600"
                    refresh={() => {
                        // Logic to refresh the assistant recommendations
                    }}
                    hasRefresh
                />
            </Section>
        </Box>
        <DialogWrapper
                open={daysDialogOpen}
                onClose={() => setDaysDialogOpen(false)}
                title="Day"
            >
                <Box className="p-4">
                </Box>
            </DialogWrapper>
        </>);
}

export default Itinerary;