import React, { useCallback, useState } from 'react';
import {
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
} from '@mui/material';
import { useDropzone } from 'react-dropzone';
import { ModalContainer } from './ModalContainer';
import { useUser } from '../../providers/UserProvider';
import { AlertDialog } from './AlertDialog';

interface Props {
  onClose: () => void;
  onUpdateSuccess: () => void;
}

export const ProfilePictureModal: React.FC<Props> = ({
  onClose,
  onUpdateSuccess
}) => {
  const [showAlert, setShowAlert] = useState(false);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const { updateProfilePicture } = useUser();

  const onDrop = useCallback((acceptedFiles: File[]) => {
    const file = acceptedFiles[0];
    if (file) {
      setSelectedFile(file);
      setPreviewUrl(URL.createObjectURL(file));
    }
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    multiple: false,
    accept: {
      'image/jpeg': ['.jpg', '.jpeg'],
      'image/png': ['.png'],
      'image/webp': ['.webp'],
    },
  });

  const handleUpload = async () => {
    if (selectedFile) {
      let success = await updateProfilePicture(selectedFile);

      if (success) {
        onUpdateSuccess()
      } else {
        setShowAlert(true);
      }
    }
  };

  return (
    <>
      <ModalContainer onClose={onClose}>
        <DialogTitle className="flex justify-between items-center">
          <span className="text-lg font-semibold">Upload Profile Picture</span>

        </DialogTitle>

        <DialogContent>
          <div
            {...getRootProps()}
            className={`border-2 border-dashed rounded-xl p-6 text-center cursor-pointer transition
            ${isDragActive ? 'border-blue-500 bg-blue-50' : 'border-gray-300'}`}
          >
            <input {...getInputProps()} />
            {previewUrl ? (
              <img
                src={previewUrl}
                alt="Preview"
                className="w-32 h-32 mx-auto object-cover rounded-full"
              />
            ) : (
              <p className="text-gray-600">
                Drag & drop an image here, or click to select a file
              </p>
            )}
          </div>
        </DialogContent>

        <DialogActions className="px-6 pb-4">
          <button className="bg-blue-500 hover:bg-blue-600 w-fit text-white px-6 rounded-md py-2 mt-3 disabled:bg-blue-800"
            disabled={!selectedFile}
            onClick={handleUpload}>
            Upload
          </button>
        </DialogActions>
      </ModalContainer>
      <AlertDialog visible={showAlert} message="There was a problem with changing your profile picture." color="text-red-500" toggleVisibility={() => setShowAlert(!showAlert)} />
    </>
  );
};


