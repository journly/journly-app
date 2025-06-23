# EncodableTripData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**banner_image** | **string** |  | [optional] [default to undefined]
**budget_plan** | [**EncodableBudgetPlan**](EncodableBudgetPlan.md) |  | [default to undefined]
**collaborators** | [**Array&lt;EncodableCollaborator&gt;**](EncodableCollaborator.md) |  | [default to undefined]
**documents** | [**Array&lt;EncodableDocument&gt;**](EncodableDocument.md) |  | [default to undefined]
**end_date** | **string** |  | [optional] [default to undefined]
**id** | **string** |  | [default to undefined]
**itinerary** | [**Array&lt;EncodableItineraryItem&gt;**](EncodableItineraryItem.md) |  | [default to undefined]
**start_date** | **string** |  | [optional] [default to undefined]
**title** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { EncodableTripData } from './api';

const instance: EncodableTripData = {
    banner_image,
    budget_plan,
    collaborators,
    documents,
    end_date,
    id,
    itinerary,
    start_date,
    title,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
