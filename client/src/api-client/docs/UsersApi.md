# UsersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**deleteUser**](#deleteuser) | **DELETE** /api/v1/users/{user_id} | |
|[**getUser**](#getuser) | **GET** /api/v1/users/{user_id} | |
|[**getUsers**](#getusers) | **GET** /api/v1/users | |
|[**updateUser**](#updateuser) | **PUT** /api/v1/users/{user_id} | |
|[**updateUserPassword**](#updateuserpassword) | **PUT** /api/v1/users/{user_id}/password | |

# **deleteUser**
> OkResponse deleteUser()


### Example

```typescript
import {
    UsersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UsersApi(configuration);

let userId: string; // (default to undefined)

const { status, data } = await apiInstance.deleteUser(
    userId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] |  | defaults to undefined|


### Return type

**OkResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful Response |  -  |
|**403** | Insufficient permissions |  -  |
|**404** | User not found |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUser**
> GetUserResponse getUser()


### Example

```typescript
import {
    UsersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UsersApi(configuration);

let userId: string; // (default to undefined)

const { status, data } = await apiInstance.getUser(
    userId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] |  | defaults to undefined|


### Return type

**GetUserResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful Response |  -  |
|**403** | Insufficient permissions |  -  |
|**404** | User Not Found |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsers**
> GetUsersResponse getUsers()


### Example

```typescript
import {
    UsersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UsersApi(configuration);

const { status, data } = await apiInstance.getUsers();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**GetUsersResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful Response |  -  |
|**403** | Insufficient permissions |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUser**
> OkResponse updateUser(updateInformationBody)


### Example

```typescript
import {
    UsersApi,
    Configuration,
    UpdateInformationBody
} from './api';

const configuration = new Configuration();
const apiInstance = new UsersApi(configuration);

let userId: string; // (default to undefined)
let updateInformationBody: UpdateInformationBody; //

const { status, data } = await apiInstance.updateUser(
    userId,
    updateInformationBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateInformationBody** | **UpdateInformationBody**|  | |
| **userId** | [**string**] |  | defaults to undefined|


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
|**200** | Successful Response |  -  |
|**403** | Insufficient permissions |  -  |
|**404** | User not found |  -  |
|**409** | Conflicts with existing resource |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUserPassword**
> OkResponse updateUserPassword(passwordUpdateRequest)


### Example

```typescript
import {
    UsersApi,
    Configuration,
    PasswordUpdateRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UsersApi(configuration);

let userId: string; // (default to undefined)
let passwordUpdateRequest: PasswordUpdateRequest; //

const { status, data } = await apiInstance.updateUserPassword(
    userId,
    passwordUpdateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **passwordUpdateRequest** | **PasswordUpdateRequest**|  | |
| **userId** | [**string**] |  | defaults to undefined|


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
|**200** | Password updated successfully |  -  |
|**403** | Unauthorized |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

