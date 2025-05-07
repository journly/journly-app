import { Stack, Tabs } from 'expo-router';
import { Dimensions, StyleSheet,View, Text } from 'react-native';
import { useColorScheme } from '@/hooks/useColorScheme';
import { Ionicons } from '@expo/vector-icons';
import { useThemeColor } from '@/hooks/useThemeColor';
import VerticalTabBar from '@/components/VerticalTabBar';

const defaultOptions = {
  headerShown: false,
  tabBarVisible: false,
  tabBarButton: () => null,
  tabbarbuttonvisible: false,
}

export default function TabLayout() {

  return (
    <>
    <View style={{flexDirection: 'row', alignItems: 'center', padding: 10}}>
      <Ionicons name="book-outline" size={30} color="BLACK" style={{ marginRight: 10 }} />
      <Text style={{ fontSize: 24, fontWeight: 'bold' }}>Journaly</Text>
      <View style={{ flexDirection: 'row', justifyContent: 'flex-end', alignItems: 'center', flex: 1 }}>
        <Ionicons name="notifications-outline" size={24} color="BLACK" style={{ marginLeft: 10 }} />
        <Ionicons name="person-outline" size={24} color="BLACK" style={{ marginLeft: 10 }} />
      </View>
    </View>
    <Tabs
      tabBar={(props) => <VerticalTabBar {...props} />}
      screenOptions={{
        headerShown: false,
      }}
    >
        <Tabs.Screen
          name="(home)"
          options={{
            title: 'Home',
            tabBarIcon: ({ color, focused }) => (
              <Ionicons name={focused ? 'home-sharp' : 'home-outline'} color={color} size={24} />
            ),
          }}
        />
        <Tabs.Screen
          name="map"
          options={{
            title: 'Map',
            tabBarIcon: ({ color, focused }) => (
              <Ionicons name={focused ? 'map' : "map-outline"} color={color} size={24}/>
            ),
          }}
        />  
        <Tabs.Screen
          name="journals"
          options={{
            title: 'Journals',
            tabBarIcon: ({ color, focused }) => (
              <Ionicons name={focused ? 'information-circle' : 'information-circle-outline'} color={color} size={24}/>
            ),
          }}
        /> 
        <Tabs.Screen
          name="explore"
          options={{
            title: 'Explore',
            tabBarIcon: ({ color, focused }) => (
              <Ionicons name={focused ? 'search' : 'search-outline'} color={color} size={24}/>
            ),
          }}
        /> 
        <Tabs.Screen
          name="itinerary"
          options={defaultOptions}
        />
        <Tabs.Screen
          name="dashboard"
          options={defaultOptions}
        />
        <Tabs.Screen
          name="budgeting"
          options={defaultOptions}
        />  
        <Tabs.Screen
          name="trip/[id]/index"
          options={defaultOptions}
        /> 
        <Tabs.Screen
          name="trip/[id]/itinerary"
          options={defaultOptions}
        /> 
        <Tabs.Screen
          name="trip/[id]/budgeting"
          options={defaultOptions}
        /> 
      </Tabs>   
      </> 
  );
}
