// src/theme/index.ts
import { createTheme } from '@mui/material/styles';

const tailwindFontFamily = [
  'ui-sans-serif',
  'system-ui',
  '-apple-system',
  'BlinkMacSystemFont',
  '"Segoe UI"',
  'Roboto',
  '"Helvetica Neue"',
  'Arial',
  '"Noto Sans"',
  'sans-serif',
  '"Apple Color Emoji"',
  '"Segoe UI Emoji"',
  '"Segoe UI Symbol"',
  '"Noto Color Emoji"',
].join(', ');

const theme = createTheme({
  palette: {
    primary: {
      main: '#2356EB', 
    },
    secondary: {
      main: '#6B7280', 
    },
    error: {
      main: '#ef4444', 
    },
    warning: {
      main: '#f59e0b', 
    },
  },
  typography: {
    fontFamily: tailwindFontFamily,
    h6: {
      fontSize: '1.5rem',
      fontWeightRegular: 550,
      textShadow: '0px 1px 2px rgba(0, 0, 0, 0.1)',
    },
    caption: {
      fontSize: '0.75rem',
      fontWeight: 400,
    },
    body2: {
      fontSize: '0.875rem',
      fontWeight: 500,
    },
  },
  components: {
    MuiButtonBase: {
      styleOverrides: {
        root: {
          '&:hover': {
            backgroundColor: 'transparent'
          },
        },
      },
    },
     MuiButton: {
      styleOverrides: {
        root: {
          gap: '0.75rem', 
          padding: '0.3rem 0.75rem', 
          textTransform: 'none', 
        },
      },
    },
  },
});

export default theme;