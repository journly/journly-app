import React from 'react';
import Router from './router/Router';
import { AuthProvider } from './providers/AuthProvider';
/**
 * App component that serves as the main entry point for the application.
 * Can put authentication logic here.
 * @returns
 */
const App: React.FC = () => {
  return (
      <Router />
  )
}

export default App;
