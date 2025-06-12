
import { Tab, Tabs, Box } from "@mui/material";
import { useState } from "react";
import PageWrapper from "../components/generic/Page";
import TabPabel from "../components/generic/TabPanel";
import { Bookings, Budget, Itinerary, Overview, Documents } from "../components/trip-details";
import TripBanner from "../components/TripBanner";
import { useTrip } from '../providers/TripProvider';
import { TripTabConfig, TripTabs } from "../components/config/TripTabConfig";


export default function TripPage() {
    const { trip } = useTrip();

    const [tabValue, setTabValue] = useState<TripTabs>(TripTabs.OVERVIEW);

    return (
        <>
        <header className="full-width border-b border-gray-200 bg-white">
            <TripBanner />
            <div className='px-6 max-w-screen-2xl mx-auto'>
                <div className="flex items-center justify-between">
                    <Tabs value={tabValue} onChange={(_, val) => setTabValue(val)}>
                        {Object.values(TripTabConfig).map((tab, i) => (
                            <Tab 
                                key={tab.value} 
                                value={tab.value} 
                                label={tab.label} 
                                className="normal-case" 
                                color="primary" 
                                icon={tab.icon} 
                                iconPosition="start" 
                                disableRipple
                                sx={{
                                    textTransform: 'none',
                                    minHeight: 36, 
                                    paddingBottom: '0px',
                                }}
                            />
                    ))}
                    </Tabs> 
                </div>
            </div>
        </header>
        <PageWrapper>
            <TabPabel value={tabValue} index={TripTabs.OVERVIEW}>
                <Overview/>
            </TabPabel>
            <TabPabel value={tabValue} index={TripTabs.ITINERARY}>
                <Itinerary />
            </TabPabel>
            <TabPabel value={tabValue} index={TripTabs.BOOKING}>
                <Bookings />
            </TabPabel>
            <TabPabel value={tabValue} index={TripTabs.BUDGET}>
                <Budget />
            </TabPabel>
            <TabPabel value={tabValue} index={TripTabs.DOCUMENTS}>
                <Documents />
            </TabPabel>
        </PageWrapper>
    </>);
}