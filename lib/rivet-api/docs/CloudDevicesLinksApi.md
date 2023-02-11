# \CloudDevicesLinksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_devices_links_complete**](CloudDevicesLinksApi.md#cloud_devices_links_complete) | **POST** /devices/links/complete | 
[**cloud_devices_links_get**](CloudDevicesLinksApi.md#cloud_devices_links_get) | **GET** /devices/links | 
[**cloud_devices_links_prepare**](CloudDevicesLinksApi.md#cloud_devices_links_prepare) | **POST** /devices/links | 



## cloud_devices_links_complete

> cloud_devices_links_complete(cloud_devices_complete_device_link_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_devices_complete_device_link_input** | [**CloudDevicesCompleteDeviceLinkInput**](CloudDevicesCompleteDeviceLinkInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_devices_links_get

> crate::models::CloudDevicesGetDeviceLinkOutput cloud_devices_links_get(device_link_token, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_link_token** | **String** |  | [required] |
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudDevicesGetDeviceLinkOutput**](CloudDevicesGetDeviceLinkOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_devices_links_prepare

> crate::models::CloudDevicesPrepareDeviceLinkOutput cloud_devices_links_prepare()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CloudDevicesPrepareDeviceLinkOutput**](CloudDevicesPrepareDeviceLinkOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
