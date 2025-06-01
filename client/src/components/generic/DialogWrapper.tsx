import React from "react";
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions
} from "@mui/material";
import { CrossIcon, X } from "lucide-react";

type DialogWrapperProps = {
  open: boolean;
  onClose: () => void;
  title?: string;
  description?: string;
  children?: React.ReactNode;
  actions?: React.ReactNode;
  maxWidth?: "sm" | "md" | "lg" | "xl";
  fullWidth?: boolean;
};

const DialogWrapper: React.FC<DialogWrapperProps> = ({
  open,
  onClose,
  title,
  description,
  children,
  actions,
  maxWidth = "sm",
  fullWidth = false
}) => {
  return (
    <Dialog open={open} onClose={onClose} fullWidth={fullWidth} maxWidth={maxWidth}>
      {title && <DialogTitle>{title}</DialogTitle>}
      <button
        className="absolute top-2 right-2 text-gray-500 hover:text-gray-700"
        onClick={onClose}
        aria-label="Close dialog"
      >
        <X className="h-5 w-5 m-2" />
      </button>
      <DialogContent>
        {description && (
          <DialogContentText>{description}</DialogContentText>
        )}
        {children}
      </DialogContent>
      {actions && <DialogActions>{actions}</DialogActions>}
    </Dialog>
  );
};

export default DialogWrapper;