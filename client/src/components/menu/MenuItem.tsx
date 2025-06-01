import { Box, ButtonBase, styled, Typography } from "@mui/material";
import { NavLink } from "react-router-dom";
import { cloneElement } from "react";
type MenuItemProps = {
  icon: React.ReactElement;
  label?: string;
  smallLabel?: string;
  link?: string;
  textColor?: string;
  iconColor?: string;
  onClick?: () => void;
};

export function MenuItem({
  icon,
  label,
  smallLabel,
  link,
  textColor,
  iconColor,
  onClick
}: MenuItemProps) {
  
  const StyledMenuItem = styled(ButtonBase)<{ active: boolean }>(({ active }) => ({
    display: 'flex',
    alignItems: 'center',
    justifyContent: "flex-start",
    gap: '0.75rem',
    padding: '0.5rem 0.75rem',
    borderRadius: '0.4rem',
    width: '100%',
    marginBottom: '0.25rem',
    fontSize: '0.875rem',
    lineHeight: '1.25rem',
    color: active ? '#2563eb' : '#374151',
    fontWeight: 300,
    backgroundColor: active ? '#eff6ff' : 'transparent',
    textAlign: 'left',
    transition: 'background-color 0.2s ease',
  }));


  return (
    <NavLink
      to={link}
    >
      {({ isActive }) => (
      <StyledMenuItem active={link ? isActive : false} onClick={onClick}>
        <span className={iconColor ?? (link && isActive ? 'text-blue-600' : 'text-gray-500')}>
            {cloneElement(icon, {
            size: 18
          })}
          </span>
          {label && (
          <Box className="flex-1 text-left">
            <Typography variant="body2" color={textColor}>{label}</Typography>
            {smallLabel && (
              <span className="text-xs text-gray-500">
                {smallLabel}
              </span>
            )}
          </Box>
          )}
      </StyledMenuItem>
      )}
    </NavLink>
  )
}