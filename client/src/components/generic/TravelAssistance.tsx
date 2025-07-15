import { Typography, Box, List, Button } from "@mui/material";
import { SparklesIcon} from "lucide-react";
import { useEffect, useState } from "react";

interface AssistantProps {
    iconColor: string;
    title?: string;
    context?: string;
    refresh?: () => void;
    hasRefresh?: boolean;
}

const TravelAssistance: React.FC<AssistantProps> = ({
    iconColor = "text-red-400",
    title = "Trip Assistant",
    context,
    refresh = () => {},
    hasRefresh = false
}: AssistantProps) => {
    //example recommendations, this could be fetched from an API or generated based on context
    const [recommendations, setRecommendations] = useState<string[]>([]);

    const handleRefresh = () => {
        // Logic to refresh the assistant recommendations
        refresh();
        // Example: Fetch new recommendations based on context
        setRecommendations([
            "Explore local cuisine in your destination",
            "Check out popular tourist attractions",
            "Consider booking guided tours for a better experience",
            "Don't forget to pack essentials like sunscreen and travel adapters"
        ]);
    }
//    useEffect(() => {
//         setRecommendations([])
//    }, [context]);

return (
    <Box className="m-0 mb-2">
        <Box className="flex items-center gap-2 m-0 mb-2">
            <SparklesIcon className={"h-5 w-5 " + iconColor} />
            <Typography variant="h5" fontWeight={500} fontSize={20} className="text-gray-900 mb-2">
                {title}
            </Typography>
        </Box>
        <Box className="mb-2 flex gap-2 overflow-y-auto max-h-60">
            <List className="flex-1">
                {recommendations.length > 0 ? (
                    recommendations.map((rec, index) => (
                        <Box key={index} className="p-2 mb-2 flex gap-2 bg-gray-100 rounded-md">
                            <Typography variant="body2" fontWeight={400} className="text-gray-600">
                                {rec}
                            </Typography>
                        </Box>
                    ))
                ) : (
                <Box className="p-2 mb-2 flex gap-2 bg-gray-100 rounded-md">
                    <Typography variant="body2" fontWeight={400} className="text-gray-600">
                        Need help planning your trip? Ask our AI assistant for suggestions!
                    </Typography>
                </Box>
                )}
            </List>
        </Box>
        {hasRefresh && (
            <Box className="flex justify-center mb-2">
                <Button
                    variant="text"
                    style={{
                        color: '#2563EB', 
                        fontWeight: 500, 
                        textTransform: 'none', 
                    }}
                    onClick={handleRefresh}
                    >
                    Get more recommendations
                </Button>
            </Box>
        )}
    </Box>
)}

export default TravelAssistance;