import { useTrip } from "../../providers/TripProvider";
import { Box, Checkbox, List, Typography } from "@mui/material";
import Section from "../generic/Section.tsx";
import { CalendarDaysIcon, DollarSign, House, MapIcon, MapPinIcon, Plus, Users } from "lucide-react";
import DialogWrapper from "../generic/DialogWrapper.tsx";
import { useState } from "react";

const InfoBox = ({title, icon, information}: { title: string; icon: any; information: string }) => (
    <Box className="flex flex-col gap-1 pt-5 row-span-1">
        <Typography variant="body1" className="text-gray-600" fontWeight={500}>
            {title}
        </Typography>
        <Box className="flex items-center gap-2 text-gray-600">
            {icon}
            <p>{information}</p>
        </Box>
    </Box>
);


const Overview = () =>{
    const { trip } = useTrip();
    const [openTaskDialog, setOpenTaskDialog] = useState<boolean>(false);
    const tasks = [
        { task: "Book flights", completed: false },
        { task: "Reserve hotel", completed: true },
        { task: "Plan itinerary", completed: false },
        { task: "Pack bags", completed: false }
    ]

    const startDate = trip?.travelDates.startDate;
    const endDate = trip?.travelDates.endDate;
    
    return (
        <>
        <Box className="grid grid-cols-3 md:grid-cols-5 gap-4 ">

            <Section additionalStyle="col-span-2 md:col-span-3 mt-0" >
                <Box className="flex items-center gap-2 mt-0">
                    <Typography variant="h6" fontWeight={500} className="text-gray-900">
                        Trip Overview
                    </Typography>
                </Box>
                <Box className="grid grid-cols-2 gap-4 mt-1">
                    <Box className="grid-rows-3 col-span-1">
                        <InfoBox 
                            title="Destination" 
                            icon={<MapPinIcon className="h-5 w-5" />} 
                            information={trip?.locations?.join(' → ') || "No locations specified"}
                        />
                        <InfoBox 
                            title="Duration" 
                            icon={<CalendarDaysIcon className="h-5 w-5" />} 
                            information={`${endDate.diff(startDate, "days").days} days (${startDate.monthShort} ${startDate.day}-${endDate.day}, ${endDate.year})` || "No dates specified"}
                        />
                        <InfoBox 
                            title="Travelers" 
                            icon={<Users className="h-5 w-5" />} 
                            information={trip?.users?.length + " people" || "No travelers specified"}
                        />
                    </Box>
                    <Box className="grid-rows-3 col-span-1">
                        <InfoBox 
                            title="Budget" 
                            icon={<DollarSign className="h-5 w-5" />} 
                            information={"No budget specified"}
                        />
                        <InfoBox 
                            title="Transportation" 
                            icon={<MapIcon className="h-5 w-5" />} 
                            information={"No transportation specified"}
                        />
                        <InfoBox 
                            title="Accomodation" 
                            icon={<House className="h-5 w-5" />} 
                            information={"No bookings specified"}
                        />
                    </Box>
                    
                                
                </Box>
            </Section>
            <Section additionalStyle="col-span-1 md:col-span-2">
                <Box className="flex justify-between gap-2 m-0">
                    <Typography variant="h6" fontWeight={500} className="text-gray-900">
                        Tasks
                    </Typography>
                    <Box className="flex items-center justify-center p-1 rounded-full cursor-pointer hover:bg-blue-100 transition-colors">
                        <Plus className="h-5 w-8 text-blue-600" onClick={() => setOpenTaskDialog(true)} />
                    </Box>
                </Box>
                <List className="mt-1 text-gray-600">
                    {tasks.map((task: any, index : any) => (
                        <Box key={index} className="flex items-center">
                            <Checkbox
                                checked={task.completed}
                                onChange={() => {
                                    // Handle task completion toggle
                                    task.completed = !task.completed;
                                }}
                                color="primary"
                            />
                            <p className={task.completed ? "text-gray-400" : ""}>
                                {task.task}
                            </p>
                        </Box>
                    ))}
                </List>
                <DialogWrapper
                    open={openTaskDialog}
                    onClose={() => setOpenTaskDialog(false)}
                    title="Create Task"
                    description="Add a new task to your trip."
                />
            </Section>
        </Box>
        <Box className="grid grid-cols-3 md:grid-cols-5 gap-4 mt-4">
            <Section additionalStyle="col-span-2 md:col-span-3" >
d
            </Section>
            <Section additionalStyle="col-span-1 md:col-span-2">
            s
            </Section>
            </Box>
        </>
    );
}

export default Overview;