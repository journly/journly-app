import { TripTabs } from "../config/TripTabConfig";

interface TabPanelProps {
  children?: React.ReactNode;
  index: TripTabs;
  value: TripTabs
}

const TabPanel: React.FC<TabPanelProps> =({ children, value, index }: TabPanelProps) => {
  return (
    <div hidden={value !== index} className="px-2">
      {value === index && <>{children}</>}
    </div>
  );
}

export default TabPanel;