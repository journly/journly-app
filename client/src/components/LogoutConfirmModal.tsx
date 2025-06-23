import React from 'react';
import { createPortal } from 'react-dom';

interface LogoutConfirmModalProps {
  isOpen: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export const LogoutConfirmModal: React.FC<LogoutConfirmModalProps> = ({ isOpen, onConfirm, onCancel }) => {
  if (!isOpen) return null;

  const root = document.getElementById('root');

  if (!root) return null;

  return (
    createPortal(
      <div className="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
        <div className="bg-white rounded-2xl shadow-xl p-6 w-full max-w-sm">
          <h2 className="text-lg font-semibold text-gray-800 mb-2">Confirm Logout</h2>
          <p className="text-sm text-gray-600 mb-6">Are you sure you want to log out?</p>

          <div className="flex justify-end space-x-3">
            <button
              onClick={onCancel}
              className="px-4 py-2 rounded-lg text-sm text-gray-600 hover:bg-gray-100 transition"
            >
              Cancel
            </button>
            <button
              onClick={onConfirm}
              className="px-4 py-2 rounded-lg text-sm text-white bg-red-600 hover:bg-red-700 transition"
            >
              Log out
            </button>
          </div>
        </div>
      </div>,
      root
    )
  );
};


