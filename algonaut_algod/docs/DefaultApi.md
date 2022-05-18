# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_application_information**](DefaultApi.md#account_application_information) | **GET** /v2/accounts/{address}/applications/{application-id} | Get account information about a given app.
[**account_asset_information**](DefaultApi.md#account_asset_information) | **GET** /v2/accounts/{address}/assets/{asset-id} | Get account information about a given asset.
[**account_information**](DefaultApi.md#account_information) | **GET** /v2/accounts/{address} | Get account information.
[**get_application_by_id**](DefaultApi.md#get_application_by_id) | **GET** /v2/applications/{application-id} | Get application information.
[**get_asset_by_id**](DefaultApi.md#get_asset_by_id) | **GET** /v2/assets/{asset-id} | Get asset information.
[**get_block**](DefaultApi.md#get_block) | **GET** /v2/blocks/{round} | Get the block for the given round.
[**get_pending_transactions**](DefaultApi.md#get_pending_transactions) | **GET** /v2/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool.
[**get_pending_transactions_by_address**](DefaultApi.md#get_pending_transactions_by_address) | **GET** /v2/accounts/{address}/transactions/pending | Get a list of unconfirmed transactions currently in the transaction pool by address.
[**get_proof**](DefaultApi.md#get_proof) | **GET** /v2/blocks/{round}/transactions/{txid}/proof | Get a Merkle proof for a transaction in a block.
[**get_status**](DefaultApi.md#get_status) | **GET** /v2/status | Gets the current node status.
[**get_supply**](DefaultApi.md#get_supply) | **GET** /v2/ledger/supply | Get the current supply reported by the ledger.
[**pending_transaction_information**](DefaultApi.md#pending_transaction_information) | **GET** /v2/transactions/pending/{txid} | Get a specific pending transaction.
[**raw_transaction**](DefaultApi.md#raw_transaction) | **POST** /v2/transactions | Broadcasts a raw transaction to the network.
[**teal_compile**](DefaultApi.md#teal_compile) | **POST** /v2/teal/compile | Compile TEAL source code to binary, produce its hash
[**teal_disassemble**](DefaultApi.md#teal_disassemble) | **POST** /v2/teal/disassemble | Disassemble program bytes into the TEAL source code.
[**teal_dryrun**](DefaultApi.md#teal_dryrun) | **POST** /v2/teal/dryrun | Provide debugging information for a transaction (or group).
[**transaction_params**](DefaultApi.md#transaction_params) | **GET** /v2/transactions/params | Get parameters for constructing a new transaction
[**wait_for_block**](DefaultApi.md#wait_for_block) | **GET** /v2/status/wait-for-block-after/{round} | Gets the node status after waiting for the given round.



## account_application_information

> crate::models::InlineResponse200 account_application_information(address, application_id, format)
Get account information about a given app.

Given a specific account public key and application ID, this call returns the account's application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application's creator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**application_id** | **i32** | An application identifier | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_asset_information

> crate::models::InlineResponse2001 account_asset_information(address, asset_id, format)
Get account information about a given asset.

Given a specific account public key and asset ID, this call returns the account's asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset's creator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**asset_id** | **i32** | An asset identifier | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_information

> crate::models::Account account_information(address, format, exclude)
Get account information.

Given a specific account public key, this call returns the accounts status, balance and spendable amounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |
**exclude** | Option<**String**> | When set to `all` will exclude asset holdings, application local state, created asset parameters, any created application parameters. Defaults to `none`. |  |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_by_id

> crate::models::Application get_application_by_id(application_id)
Get application information.

Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** | An application identifier | [required] |

### Return type

[**crate::models::Application**](Application.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_by_id

> crate::models::Asset get_asset_by_id(asset_id)
Get asset information.

Given a asset ID, it returns asset information including creator, name, total supply and special addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **i32** | An asset identifier | [required] |

### Return type

[**crate::models::Asset**](Asset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block

> crate::models::InlineResponse2003 get_block(round, format)
Get the block for the given round.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round from which to fetch block information. | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_transactions

> crate::models::InlineResponse2002 get_pending_transactions(max, format)
Get a list of unconfirmed transactions currently in the transaction pool.

Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Truncated number of transactions to display. If max=0, returns all pending txns. |  |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_transactions_by_address

> crate::models::InlineResponse2002 get_pending_transactions_by_address(address, max, format)
Get a list of unconfirmed transactions currently in the transaction pool by address.

Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | An account public key | [required] |
**max** | Option<**i32**> | Truncated number of transactions to display. If max=0, returns all pending txns. |  |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_proof

> crate::models::InlineResponse2004 get_proof(round, txid, format)
Get a Merkle proof for a transaction in a block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round in which the transaction appears. | [required] |
**txid** | **String** | The transaction ID for which to generate a proof. | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> crate::models::InlineResponse2009 get_status()
Gets the current node status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supply

> crate::models::InlineResponse2007 get_supply()
Get the current supply reported by the ledger.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pending_transaction_information

> crate::models::PendingTransactionResponse pending_transaction_information(txid, format)
Get a specific pending transaction.

Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed: - transaction committed (committed round > 0) - transaction still in the pool (committed round = 0, pool error = \"\") - transaction removed from pool due to error (committed round = 0, pool error != \"\") Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | A transaction ID | [required] |
**format** | Option<**String**> | Configures whether the response object is JSON or MessagePack encoded. |  |

### Return type

[**crate::models::PendingTransactionResponse**](PendingTransactionResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## raw_transaction

> crate::models::InlineResponse20014 raw_transaction(rawtxn)
Broadcasts a raw transaction to the network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rawtxn** | **std::path::PathBuf** | The byte encoded signed transaction to broadcast to network | [required] |

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/x-binary
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teal_compile

> crate::models::InlineResponse20011 teal_compile(source)
Compile TEAL source code to binary, produce its hash

Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **std::path::PathBuf** | TEAL source code to be compiled | [required] |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teal_disassemble

> crate::models::InlineResponse20012 teal_disassemble(source)
Disassemble program bytes into the TEAL source code.

Given the base64 encoded program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **String** | TEAL program binary to be disassembled | [required] |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/x-binary
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teal_dryrun

> crate::models::InlineResponse20013 teal_dryrun(request)
Provide debugging information for a transaction (or group).

Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | Option<[**DryrunRequest**](DryrunRequest.md)> | Transaction (or group) and any accompanying state-simulation data. |  |

### Return type

[**crate::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json, application/msgpack
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_params

> crate::models::InlineResponse20015 transaction_params()
Get parameters for constructing a new transaction

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wait_for_block

> crate::models::InlineResponse2009 wait_for_block(round)
Gets the node status after waiting for the given round.

Waits for a block to appear after round {round} and returns the node's status at the time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**round** | **i32** | The round to wait until returning status | [required] |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

