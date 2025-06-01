# Expo Routing
> This document will explain how the current navigation and routing run within out journaly system.

## Pages
The structure of the application will be as followed:
```
app
-> _layout.tsx
-> about.tsx
-> +not-found.tsx_
-> (tabs)
    -> _layout.tsx
    -> index.tsx
    -> dashboard.tsx
    -> itinerary.tsx
    -> map.tsx
    -> budgeting.tsx
    -> ....
```
### 1. app
>This folder acts as a container for all navigation related pages and its formatting. Any file created within that are not index or layout are considered a page/screen within our system.

In our system we want to aim to keep this directory strict to pages that don't need the tabs page, such as the not found or abouts page. For the future we could also have our user profile or settings pages nested here.


### 2. tabs
> This contains any pages that can be seem within the tab view of any page in the system, those that are not in the folder will not have a tab visible to them. 

In this directory we can keep every single screen to be part of the tab views, whether they are part of the tab menu or exclude it can be configured in the `_layout.tsx`

### 3. _layout.tsx
> A wrapper that encapsulates any shared UI elements such as headers or tab bars allowing them to be consistent throughout all views.
#### 3.1. app/_layout.tsx
This layout file will hold all components and elements on the screen that we want to keep all the time, this may include our tabs, user profile picture and even our logo.

#### 3.2. (tabs)/_layout.tsx
This layout file will hold all the tabs we want to style, manipulate and exclude within our application.

*Tab Format - view more standards in the [documentation](https://docs.expo.dev/router/advanced/tabs/)*
```typescript
<Tabs screenOptions={{ tabBarActiveTintColor: 'blue' }}>
    <Tabs.Screen
        name="<page>"
        options={{
            title: ....
            tabBarIcon: ...
        }}
        href=...
    />
    ...
</Tabs>
```
*Excluding Screens From the manu bar*
```typescript
<Tabs.Screen
    name="<page>"
    options={{
        href: null,
    }}
    />
</Tabs>
```

### 4. index.tsx
> This acts as the home page of our system and can be altered to later include the first time global view.

On this page we will have a summary of all our trips.

### {page}.tsx

> These pages/screens act as the routes of our system, with each having their own path in the application.

e.g. `explore.txs` would have the path of `..../explore`.

