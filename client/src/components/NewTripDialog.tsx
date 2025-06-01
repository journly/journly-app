import * as Yup from "yup";
import FormWrapper from "./generic/FormWrapper";
import FormTextField from "./generic/FormTextField";
import DialogWrapper from "./generic/DialogWrapper";

type TripFormValues = {
  title: string;
  travelDates: string; 
};

const validationSchema = Yup.object({
  title: Yup.string().required("Trip title is required"),
  travelDates: Yup.string().required("dates are required")
});

interface CreateTripFormProps {
    open: boolean;
    onClose: () => void;
}


const CreateTripForm = ({open, onClose}: CreateTripFormProps) => {
  const initialValues: TripFormValues = {
    title: "",
    travelDates: ""
  };

  const handleSubmit = async (values: TripFormValues) => {
    
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
            validationSchema={validationSchema}
            onSubmit={handleSubmit}
            submitLabel="Create Trip"
        >
            <p>Fill in the details below to create a new trip.</p>
            <FormTextField name="title" label="Trip Title" required/>
            <FormTextField name="travelDates" label="Dates" required/>
            <FormTextField name="participants" label="Invite Participants (emails)" multiline rows={3} />
        </FormWrapper>
    </DialogWrapper>
  );
};

export default CreateTripForm;
