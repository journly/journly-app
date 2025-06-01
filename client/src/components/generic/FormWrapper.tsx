import React from "react";
import { Formik, Form, FormikHelpers, FormikConfig, FormikValues } from "formik";
import { Box, Button, Typography } from "@mui/material";

type FormWrapperProps<T> = {
  title?: string;
  initialValues: T;
  validationSchema?: FormikConfig<T>["validationSchema"];
  onSubmit: (values: T, formikHelpers: FormikHelpers<T>) => void | Promise<any>;
  children: React.ReactNode;
  submitLabel?: string;
  actions?: React.ReactNode;
};

function FormWrapper<T extends FormikValues>({
  title,
  initialValues,
  validationSchema,
  onSubmit,
  children,
  submitLabel = "Submit",
  actions
}: FormWrapperProps<T>) {
  return (
    <Formik<T>
      initialValues={initialValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {({ isSubmitting }) => (
        <Form>
          <Box
            className="flex flex-col gap-2 py-2"
          >
            {title && <Typography variant="h6">{title}</Typography>}
            {children}

            {actions ? (
              actions
            ) : (
              <Button
                type="submit"
                variant="contained"
                disabled={isSubmitting}
              >
                {submitLabel}
              </Button>
            )}
          </Box>
        </Form>
      )}
    </Formik>
  );
}

export default FormWrapper;