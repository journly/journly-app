# AuthenticationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getMe**](#getme) | **GET** /api/v1/auth/me | |
|[**login**](#login) | **POST** /api/v1/auth/login | |
|[**logout**](#logout) | **POST** /api/v1/auth/logout | |
|[**refresh**](#refresh) | **POST** /api/v1/auth/refresh | |
|[**registerUser**](#registeruser) | **POST** /api/v1/auth/register | |

# **getMe**
> GetMeResponse getMe()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.getMe();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**GetMeResponse**

### Authorization

[jwt](../README.md#jwt)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successful response |  -  |
|**404** | User not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **login**
> LoginResponse login(loginCredentials)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    LoginCredentials
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let loginCredentials: LoginCredentials; //

const { status, data } = await apiInstance.login(
    loginCredentials
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **loginCredentials** | **LoginCredentials**|  | |


### Return type

**LoginResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Login was successful |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **logout**
> OkResponse logout(refreshTokenBody)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    RefreshTokenBody
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let refreshTokenBody: RefreshTokenBody; //

const { status, data } = await apiInstance.logout(
    refreshTokenBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **refreshTokenBody** | **RefreshTokenBody**|  | |


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
|**200** | Logout was successful |  -  |
|**401** | Invalid or missing token |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refresh**
> RefreshResponse refresh(refreshTokenBody)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    RefreshTokenBody
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let refreshTokenBody: RefreshTokenBody; //

const { status, data } = await apiInstance.refresh(
    refreshTokenBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **refreshTokenBody** | **RefreshTokenBody**|  | |


### Return type

**RefreshResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Refresh was succesful |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **registerUser**
> RegisterUserResponse registerUser(registerUserBody)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    RegisterUserBody
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let registerUserBody: RegisterUserBody; //

const { status, data } = await apiInstance.registerUser(
    registerUserBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerUserBody** | **RegisterUserBody**|  | |


### Return type

**RegisterUserResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully registered |  -  |
|**400** | Invalid registration details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

