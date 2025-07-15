import * as Yup from "yup";
import FormWrapper from "./generic/FormWrapper";
import FormTextField from "./generic/FormTextField";
import DialogWrapper from "./generic/DialogWrapper";
import { CreateTrip, EncodableTripData, TripsApi } from "../api-client";
import { DatePicker, LocalizationProvider } from "@mui/x-date-pickers";
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { useFormikContext } from 'formik';
import { Box, TextField } from "@mui/material";
import { ArrowBigRightIcon, ArrowRightSquareIcon } from "lucide-react";
import { useAuth } from "../providers/AuthProvider";

type TripFormValues = {
  title: string;
  startDate: Date | null;
  endDate: Date | null;
};

const validationSchema = Yup.object({
  title: Yup.string().required("Trip title is required"),
  travelDates: Yup.string().required("dates are required")
});

interface CreateTripFormProps {
  open: boolean;
  onClose: () => void;
}

const tripsApi = new TripsApi();

const CreateTripForm = ({ open, onClose }: CreateTripFormProps) => {
  const { getUser } = useAuth();

  const initialValues: TripFormValues = {
    title: "",
    startDate: new Date(),
    endDate: new Date(),
  };

  const handleSubmit = async (values: TripFormValues) => {
    console.log("Trip created successfully");
    await tripsApi.createTrip({
      createTrip: {
        title: values.title,
        startDate: values.startDate,
        endDate: values.endDate,
        userId: getUser()?.id ?? ""
      } as CreateTrip
    });
  };

  return (
    <DialogWrapper
      open={open}
      onClose={onClose}
      fullWidth
    >
      <FormWrapper<TripFormValues>
        title="Create New Trip"
        initialValues={initialValues}
        // validationSchema={validationSchema}
        onSubmit={handleSubmit}
        submitLabel="Create Trip"
      >
        <p>Fill in the details below to create a new trip.</p>
        <TripFields />
      </FormWrapper>
    </DialogWrapper>
  );
};

const TripFields = () => {
  const { values, setFieldValue } = useFormikContext<TripFormValues>();

  return (
    <>
      <FormTextField name="title" label="Trip Title" required />
      <Box className="flex flex-auto justify-between ">
        <LocalizationProvider dateAdapter={AdapterDateFns}>
          <DatePicker
            label="Start Date"
            value={values.startDate}
            onChange={(val) => setFieldValue('startDate', val)}
          />
          <ArrowBigRightIcon className="text-gray-500 m-4" />
          <DatePicker
            label="End Date"
            value={values.endDate}
            onChange={(val) => setFieldValue('endDate', val)}
          />
        </LocalizationProvider>
      </Box>
      <FormTextField name="participants" label="Invite Participants (emails)" multiline rows={3} />
    </>
  );
};

export default CreateTripForm;
