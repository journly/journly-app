# TripsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createTrip**](#createtrip) | **POST** /api/v1/trips | |
|[**getTrip**](#gettrip) | **GET** /api/v1/trips/{trip_id} | |
|[**getTrips**](#gettrips) | **GET** /api/v1/trips | |

# **createTrip**
> OkResponse createTrip(createTripBody)


### Example

```typescript
import {
    TripsApi,
    Configuration,
    CreateTripBody
} from './api';

const configuration = new Configuration();
const apiInstance = new TripsApi(configuration);

let createTripBody: CreateTripBody; //

const { status, data } = await apiInstance.createTrip(
    createTripBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createTripBody** | **CreateTripBody**|  | |


### Return type

**OkResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Trip was created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTrip**
> GetTripResponse getTrip()


### Example

```typescript
import {
    TripsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TripsApi(configuration);

let tripId: string; // (default to undefined)

const { status, data } = await apiInstance.getTrip(
    tripId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tripId** | [**string**] |  | defaults to undefined|


### Return type

**GetTripResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Trip was found |  -  |
|**401** | User unauthorised to get trip |  -  |
|**404** | Trip not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTrips**
> GetTripsResponse getTrips()


### Example

```typescript
import {
    TripsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TripsApi(configuration);

const { status, data } = await apiInstance.getTrips();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**GetTripsResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Trips were found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

