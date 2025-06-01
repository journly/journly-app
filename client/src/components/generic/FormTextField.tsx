import React from "react";
import { TextField } from "@mui/material";
import { useFormikContext } from "formik";

type FormTextFieldProps = {
    name: string;
    label: string;
    type?: string;
    fullWidth?: boolean;
    multiline?: boolean;
    rows?: number;
    required?: boolean;
    helperText?: string; 
};

const FormTextField: React.FC<FormTextFieldProps> = ({
    name,
    label,
    type = "text",
    fullWidth = true,
    multiline = false,
    rows,
    required = false,
    helperText = "field required"
}) => {
    const { values, touched, errors, handleChange, handleBlur } = useFormikContext<any>();

    const fieldError = required && touched[name] && Boolean(errors[name]);
    const requiredMessage = required && touched[name] && helperText

    return (
        <TextField
        fullWidth={fullWidth}
        multiline={multiline}
        rows={rows}
        type={type}
        name={name}
        label={label}
        value={values[name] ?? ""}
        onChange={handleChange}
        onBlur={handleBlur}
        error={fieldError}
        helperText={requiredMessage}
        required={required}
        variant="outlined"
        />
    );
};

export default FormTextField;
