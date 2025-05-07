import { View } from "react-native";

export default function DashboardScreen() {
  return (
    <View style={{ flex: 1, justifyContent: "center", alignItems: "center" }}>
      <h1>Dashboard</h1>
      <p>Welcome to the Dashboard!</p>
      <p>This is where you can view your trip details and statistics.</p>
    </View>
  );
}