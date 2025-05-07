import { BottomTabBarProps } from '@react-navigation/bottom-tabs';
import { View, Text, TouchableOpacity, StyleSheet, Dimensions } from 'react-native';
import { Tooltip } from 'react-native-elements';

const { height } = Dimensions.get('window');

export default function VerticalTabBar({ state, descriptors, navigation }: BottomTabBarProps) {
  return (
    <View style={styles.container}>
      {state.routes.map((route, index) => {
        const { options } = descriptors[route.key];
        const isFocused = state.index === index;

        const onPress = () => {
          const event = navigation.emit({
            type: 'tabPress',
            target: route.key,
            canPreventDefault: true,
          });

          if (!isFocused && !event.defaultPrevented) {
            navigation.navigate(route.name);
          }
        };

        // Get tabBarIcon
        const icon = options.tabBarIcon?.({
          focused: isFocused,
          color: isFocused ? '#000' : '#888',
          size: 24,
        });

        return (
          <TouchableOpacity
            key={route.key}
            accessibilityRole="button"
            onPress={onPress}
            style={[styles.tabButton, isFocused && styles.activeTab]}
          >
            {icon}
          </TouchableOpacity>
        );
      })}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    top: height * 0.3,
    left: 20,
    width: 54,
    backgroundColor: '#fff',
    borderRadius: 20,
    paddingVertical: 20,
    justifyContent: 'space-around',
    alignItems: 'flex-start',
    elevation: 5,
  },
  tabButton: {
    paddingVertical: 12,
    paddingHorizontal: 15,
    flexDirection: 'row',
    alignItems: 'center',
  },
  tabText: {
    fontSize: 14,
    color: '#222',
  },
  activeTab: {
    backgroundColor: '#eee',
    borderRadius: 10,
  },
  activeText: {
    color: '#673ab7',
    fontWeight: 'bold',
  },
  inactiveText: {
    color: '#888',
  },
});
