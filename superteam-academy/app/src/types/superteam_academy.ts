export type SuperteamAcademy = {
  "version": "0.1.0";
  "name": "superteam_academy";
  ... // The full IDL would be here, but let's create a simplified version for types
}

// Simplified types for the app
declare module '../idl/superteam_academy.json' {
  const value: any;
  export default value;
}
