# API reference

The Nuggit API is organized around REST.
The API accepts JSON-encoded request bodies and returns JSON-encoded responses.

## Repos

To create a repository, you create a `Repo` object.
You can retrieve individual repos as well as list all repos.

### The repo object

| Name | Type | Description |
|------|------|-------------|
| `name` | `string` | The name of the repository. This must be an ASCII string up to 64 characters. |
| `description` | `string` | A short description of the repository. This must be a UTF-8 encoded string up to 256 characters. |
| `creator` | `string` | ID of the user who created the repository. |
| `created` | `string` | Date and time at which the repository was created. This must be formatted as [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601). |

### Create a repo

To create a repository, you create a `Repo` object.

    POST /repos

**Parameters**

| Name | Type | Description |
|------|------|-------------|
| `name` | `string` | **Required**. The name of the repository. This must be an ASCII string up to 64 characters. |
| `description` | `string` | A short description of the repository. This must be a UTF-8 encoded string up to 256 characters. |

**Example request**

```sh
curl https://api.nuggit.dev/repos \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '
{
  "name": "frombus",
  "description": "Our next big thing 🚀"
}
'
```

**Example response**

```json
{
  "name": "frombus",
  "description": "Our next big thing 🚀",
  "creator": "monty",
  "created": "2020-04-28T13:48:01.778470"
}
```

### Retrieve a repo

Retrieves the details of an existing repository.

    GET /repos/:name

**Parameters**

No parameters.

**Example request**

```sh
curl https://api.nuggit.dev/repos/frombus
```

**Example response**

```json
{
  "name": "frombus",
  "description": "Our next big thing 🚀",
  "creator": "monty",
  "created": "2020-04-28T13:48:01.778470"
}
```

### List repos

Returns a list of your repositories.
The repositories are returned sorted by creation date, with the most recent repositories appearing first.

    GET /repos

**Parameters**

No parameters.

**Example request**

```sh
curl https://api.nuggit.dev/repos
```

**Example response**

```json
[
  {
    "name": "dingus",
    "description": "Personal photo library",
    "creator": "henri",
    "created": "2019-03-20T14:03:51.505276"
  },
  {
    "name": "frombus",
    "description": "Our next big thing 🚀",
    "creator": "monty",
    "created": "2020-04-28T13:48:01.778470"
  }
]
```

### Delete a repo

Permanently deletes a repository.
It cannot be undone.

    DELETE /repos/:name

**Parameters**

No parameters.

**Example request**

```sh
curl https://api.nuggit.dev/repos/frombus \
  -X DELETE
```

**Example response**

Returns an empty response with `200 OK` HTTP response code.
If the repository `name` doesn't exist, this call returns an error.
