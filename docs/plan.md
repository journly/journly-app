## Future integrations
Journly should transition to a local-first architecture after core functionality has been completed. This will allow users to use the application without internet access, and also support synchronisation between local data and server-side data. Furthermore, all modifications on the client application should be reflected instantly. Conflicting modifications between two clients should be reconciled on the server-side.

In order for this to be achieved, the following features must be implemented:

Client side (sync-engine):
- [] persistent local storage
- [] data modification/deletion detection
- [] pushing updates to server
- [] receiving updates 

Server side:
- [] data validation and persistent storage
- [] conflict resolution
- [] websocket or polling based connection for real-time updates

