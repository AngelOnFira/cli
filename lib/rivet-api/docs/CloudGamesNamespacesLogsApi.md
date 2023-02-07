# \CloudGamesNamespacesLogsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_games_namespaces_logs_get_namespace_lobby**](CloudGamesNamespacesLogsApi.md#cloud_games_namespaces_logs_get_namespace_lobby) | **GET** /games/{game_id}/namespaces/{namespace_id}/logs/lobbies/{lobby_id} | 
[**cloud_games_namespaces_logs_list_namespace_lobbies**](CloudGamesNamespacesLogsApi.md#cloud_games_namespaces_logs_list_namespace_lobbies) | **GET** /games/{game_id}/namespaces/{namespace_id}/logs/lobbies | 



## cloud_games_namespaces_logs_get_namespace_lobby

> crate::models::CloudGamesNamespacesGetNamespaceLobbyOutput cloud_games_namespaces_logs_get_namespace_lobby(game_id, namespace_id, lobby_id)


Returns a lobby from the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**lobby_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesNamespacesGetNamespaceLobbyOutput**](CloudGamesNamespacesGetNamespaceLobbyOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_games_namespaces_logs_list_namespace_lobbies

> crate::models::CloudGamesNamespacesListNamespaceLobbiesOutput cloud_games_namespaces_logs_list_namespace_lobbies(game_id, namespace_id, before_create_ts)


Returns a list of lobbies for the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**before_create_ts** | Option<**String**> | Returns lobbies created before this timestamp. |  |

### Return type

[**crate::models::CloudGamesNamespacesListNamespaceLobbiesOutput**](CloudGamesNamespacesListNamespaceLobbiesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
