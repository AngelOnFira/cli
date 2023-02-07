# \MatchmakerRegionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**matchmaker_regions_list**](MatchmakerRegionsApi.md#matchmaker_regions_list) | **GET** /regions | 



## matchmaker_regions_list

> crate::models::MatchmakerListRegionsOutput matchmaker_regions_list()


Returns a list of regions available to this namespace. Regions are sorted by most optimal to least optimal. The player's IP address is used to calculate the regions' optimality. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MatchmakerListRegionsOutput**](MatchmakerListRegionsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
