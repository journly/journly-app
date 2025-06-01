//creat react component for search bar

import { InputBase, Box } from "@mui/material";
import { SearchIcon } from "lucide-react";
import { useState } from "react";

// searchbar properties
interface SearchBarProps {
    placeholder?: string;
    onSearch?: (value: string) => void;
}

export const SearchBar: React.FC<SearchBarProps> = ({ placeholder, onSearch }) => {
 const [searchValue, setSearchValue] = useState("");

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setSearchValue(value);
    if (onSearch) {
      onSearch(value); // trigger callback
    }
  };

  return (
    <Box className="px-3 py-2">
      <Box className="relative">
        {/* <SearchIcon className="absolute left-2 top-1/2 transform -translate-y-1/2 text-gray-400" /> */}
        <InputBase
          placeholder={placeholder || "Search"}
          value={searchValue}
          onChange={handleChange}
          className="w-full pl-8 pr-2 py-1.5 bg-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </Box>
    </Box>
  );
};