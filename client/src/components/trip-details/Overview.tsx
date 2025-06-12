import { useTrip } from "../../providers/TripProvider";
import { Avatar, Box, Checkbox, List, Typography } from "@mui/material";
import Section from "../generic/Section.tsx";
import { CalendarDaysIcon, DollarSign, DotIcon, House, MapIcon, MapPinIcon, Plus, SparklesIcon, Users } from "lucide-react";
import DialogWrapper from "../generic/DialogWrapper.tsx";
import { useState } from "react";

const InfoBox = ({title, icon, information}: { title: string; icon: any; information: string }) => (
    <Box className="flex flex-col gap-1 pt-5 row-span-1">
        <Typography variant="body1" className="text-gray-600" fontWeight={500} fontSize={16}>
            {title}
        </Typography>
        <Box className="flex items-center gap-2 text-gray-600 text-sm">
            {icon}
            <p>{information}</p>
        </Box>
    </Box>
);


const Overview = () =>{
    const { trip } = useTrip();
    const [openTaskDialog, setOpenTaskDialog] = useState<boolean>(false);
    const [openNoteDialog, setOpenNoteDialog] = useState<boolean>(false);
    const tasks = [
        { task: "Book flights", completed: false },
        { task: "Reserve hotel", completed: true },
        { task: "Plan itinerary", completed: false },
        { task: "Pack bags", completed: false }
    ]
    const notes = [
        { creator: "John Doe", content: "Remember to check the weather before packing.", date: new Date() },
        { creator: "Jane Smith", content: "Don't forget to bring travel insurance documents.", date: new Date() },
        { creator: "Alice Johnson", content: "Check visa requirements for Japan.", date: new Date() }
    ];

    const startDate = trip?.travelDates.startDate;
    const endDate = trip?.travelDates.endDate;
    
    return (
        <>
        <Box className="grid grid-cols-3 md:grid-cols-5 gap-4 ">

            <Section additionalStyle="col-span-2 md:col-span-3 mt-0" >
                <Box className="flex items-center gap-2 mt-0">
                    <Typography variant="h5" fontWeight={500} fontSize={20} className="text-gray-900">
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
                    <Typography variant="h5" fontWeight={500} fontSize={20} className="text-gray-900">
                        Tasks
                    </Typography>
                    <Box className="flex items-center justify-center p-1 rounded-full cursor-pointer hover:bg-blue-100 transition-colors" onClick={() => setOpenTaskDialog(true)}>
                        <Plus className="h-5 w-8 text-blue-600" />
                    </Box>
                </Box>
                <List className="mt-1 text-gray-600 text-sm overflow-y-auto max-h-48">
                    {tasks.length === 0 ? (
                        <Typography variant="body2" className="text-gray-500">
                            No tasks available. Click the plus icon to add a new task.
                        </Typography>
                    ): tasks.map((task: any, index : any) => (
                        <Box key={index} className="flex items-center">
                            <Checkbox
                                checked={task.completed}
                                onChange={() => {
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
                <Box className="flex justify-between gap-2 m-0 mb-2">
                    <Typography variant="h5" fontWeight={500} fontSize={20} className="text-gray-900">
                        Notes
                    </Typography>
                    <Box className="flex items-center justify-center p-1 rounded-full cursor-pointer hover:bg-blue-100 transition-colors" onClick={() => setOpenNoteDialog(true)}>
                        <Plus className="h-5 w-8 text-blue-600" />
                    </Box>
                </Box>
                <Box className="text-gray-600 text-sm overflow-y-auto max-h-56">
                    {notes.length === 0 ? (
                        <Typography variant="body2" className="text-gray-500">
                            No notes available. Click the plus icon to add a new note.
                        </Typography>
                    ) : (
                        <List className="mt-1 text-gray-600 text-sm overflow-y-auto max-h-48">
                        {notes.map((note, index) => (
                            <Box key={index} className="p-2 mb-2 flex gap-2 bg-gray-100 rounded-md">
                                <Avatar alt={note.creator} src={"/avatars/default.jpg"} className="mb-1 col-span-1" />
                                <Box className="mb-1 col-span-2">
                                    <Typography variant="body2" fontWeight={500}>
                                        {note.content}
                                    </Typography>
                                    <Box className="gap-2 flex">
                                        <Typography variant="caption" className="text-gray-500">
                                            {note.creator}
                                        </Typography>
                                        <DotIcon className="h-5 w-5 text-gray-400" />
                                        <Typography variant="caption" className="text-gray-500">
                                            {new Date(note.date).toLocaleDateString()}
                                        </Typography>
                                    </Box>
                                </Box>
                            </Box>
                        ))}
                        </List>
                    )}
                </Box>
                <DialogWrapper
                    open={openNoteDialog}
                    onClose={() => setOpenNoteDialog(false)}
                    title="Create Note"
                    description="Add a new note to your trip."
                />
            </Section>
            <Section additionalStyle="col-span-1 md:col-span-2">
                <Box className="m-0 mb-2">
                    <Box className="flex items-center gap-2 m-0 mb-2">
                        <SparklesIcon className="h-5 w-5 text-red-400" />
                        <Typography variant="h5" fontWeight={500} fontSize={20} className="text-gray-900 mb-2">
                            Trip Assistant
                        </Typography>
                    </Box>
                    <Box className="p-2 mb-2 flex gap-2 bg-gray-100 rounded-md">
                        <List className="flex-1">
                            <Typography variant="body2" fontWeight={400} className="text-gray-600">
                                Need help planning your trip? Ask our AI assistant for suggestions!
                            </Typography>
                        </List>
                    </Box>
                </Box>
            </Section>
            </Box>
        </>
    );
}

export default Overview;