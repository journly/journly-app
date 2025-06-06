import React from "react";
interface SectionButtonProps {
    title: string;
    icon: React.ReactNode;
    onClick: () => void;
    className?: string;
}

//TODO make all section buttons consistent
const SectionButton:  React.FC<SectionButtonProps> = ({ title, icon, onClick, className }) => {
    return (
        <button
            onClick={onClick}
            className={`flex items-center gap-2 p-2 rounded-lg hover:bg-gray-100 transition-colors ${className}`}
        >
            <span className="text-lg">{icon}</span>
            <span className="text-base font-medium">{title}</span>
        </button>
    );
}

export default SectionButton;