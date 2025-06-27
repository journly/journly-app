// create notification drawer component
import { Drawer, Typography, Box } from '@mui/material';
import { Eye, X } from 'lucide-react';
import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
interface NotificationPanelProps {
  open: boolean;
  onClose: (open: boolean) => void;
}

export const NotificationPanel: React.FC<NotificationPanelProps> = ({
  open,
  onClose
}) => {
  const [notifications, setNotifications] = React.useState<any[]>([]);

  const dismissNotification = (id: number) => {
    // Update the seen status of the notification
    setNotifications((prevNotifications) =>
      prevNotifications.map((notification) =>
        notification.id === id ? { ...notification, seen: true } : notification
      )
    );
    // update the database or API to mark the notification as seen

  }

  useEffect(() => {
    const fetchNotifications = async () => {
      // try {
      //     // Simulate fetching notifications from an API
      //     const response = await fetch('/api/notifications'); // Replace with your API endpoint
      //     const data = await response.json();
      //     setNotifications(data);
      // } catch (error) {
      //     console.error('Error fetching notifications:', error);
      // }
      setNotifications([
        { id: 1, type: "invite", message: '@Joe has invited you to "Japan 2023"', seen: false },
        { id: 2, type: "trip", message: 'Your trip to "Japan 2023" is approaching', seen: false },
        { id: 3, type: "trip", message: 'New comment on your trip "Japan 2023" by @Alice', seen: false },
        { id: 5, type: "reminder", message: 'Reminder: Your trip "Japan 2023" starts in 3 days', seen: false }
      ])
    };
    fetchNotifications();
  }, [open]);

  return (
    <Box>
      <Drawer
        anchor="left"
        open={open}
        onClose={() => onClose(false)}
        PaperProps={{
          sx: {
            width: 260,
            backgroundColor: '#f5f5f5',
            borderRight: '1px solid #ddd',
            boxShadow: '0 2px 10px rgba(0,0,0,0.1)'
          }
        }}
      >
        <Box className="p-4">
          <Typography variant="h6" className="mb-4 pb-2">
            Notifications
          </Typography>
          {notifications.length > 0 ? (
            notifications.map((notification) => (
              <Box
                key={notification.id}
                className={`p-3 mb-2 rounded-md ${notification.seen ? 'bg-gray-200' : 'bg-white hover:bg-gray-50'} flex items-center justify-between`}
                style={{ borderLeft: notification.seen ? '4px solid #ccc' : '4px solid #007bff' }}
              >
                <Typography variant="body2" >
                  {notification.message}
                </Typography>
                <div className="flex flex-col items-center gap-2">
                  {notification.type === "invite" && (
                    <Link to={`/settings`} className={`${notification.seen ? 'text-gray-500' : 'text-blue-600'}`} onClick={() => onClose}>
                      <Eye size={18} />
                    </Link>
                  )}
                  {!notification.seen && (
                    <button
                      className="relative flex justify-end text-gray-500 hover:text-gray-700"
                      onClick={() => dismissNotification(notification.id)}
                    >
                      <X size={18} />
                    </button>
                  )}
                </div>
              </Box>
            ))
          ) : (
            <Box className="p-4 text-center">
              <Typography variant="body1" color="textSecondary">
                No notifications available
              </Typography>
            </Box>
          )}
        </Box>
      </Drawer>
    </Box>
  );
}
