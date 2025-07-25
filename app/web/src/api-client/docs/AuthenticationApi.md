# AuthenticationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getMe**](#getme) | **GET** /api/v1/auth/me | |
|[**login**](#login) | **POST** /api/v1/auth/login | |
|[**logout**](#logout) | **POST** /api/v1/auth/logout | |
|[**refresh**](#refresh) | **POST** /api/v1/auth/refresh | |
|[**registerUser**](#registeruser) | **POST** /api/v1/auth/register | |
|[**resendVerificationCode**](#resendverificationcode) | **POST** /api/v1/auth/resend-verification | |
|[**verifyUserEmail**](#verifyuseremail) | **POST** /api/v1/auth/verify-email | |

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
|**401** | Unauthorized |  -  |
|**500** | Internal error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **logout**
> OkResponse logout()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.logout();
```

### Parameters
This endpoint does not have any parameters.


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
|**200** | Logout was successful |  -  |
|**401** | Invalid or missing token |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refresh**
> RefreshResponse refresh()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.refresh();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**RefreshResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Refresh was succesful |  -  |
|**401** | Unauthorized |  -  |
|**500** | Internal error |  -  |

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

# **resendVerificationCode**
> OkResponse resendVerificationCode(resendVerificationBody)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    ResendVerificationBody
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let resendVerificationBody: ResendVerificationBody; //

const { status, data } = await apiInstance.resendVerificationCode(
    resendVerificationBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **resendVerificationBody** | **ResendVerificationBody**|  | |


### Return type

**OkResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Verification code resent |  -  |
|**400** | Invalid request |  -  |
|**404** | User not found |  -  |
|**409** | User already verified |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verifyUserEmail**
> OkResponse verifyUserEmail(verificationBody)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    VerificationBody
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let verificationBody: VerificationBody; //

const { status, data } = await apiInstance.verifyUserEmail(
    verificationBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **verificationBody** | **VerificationBody**|  | |


### Return type

**OkResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Email successfully verified |  -  |
|**400** | Invalid request |  -  |
|**403** | Verification code is incorrect or expired |  -  |
|**404** | User not found |  -  |
|**409** | User already verified |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

