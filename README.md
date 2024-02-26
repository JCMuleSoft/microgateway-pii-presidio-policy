# PII Policy

This policy uses [presidio](https://microsoft.github.io/presidio/) to detect sensitive data being sent on the body.

## Configuration

| Name            | Type   | Description                                                            | Default Value |
|-----------------|--------|------------------------------------------------------------------------|---------------|
| language        | string | Two characters for the desired language in ISO_639-1 format            | en            |
| score_threshold | number | The minimal detection score threshold                                  | 0.5           |
| action          | string | The action to do in case the request is consider it has sensitive data | Reject        |
| server          | string | The name of the service where presidio is running                      | presidio      |
| server_url      | string | The url of the presidio server                                         |               |

## Example

### Sensitive data request
```
> curl -d '{"name":"John Smith", "driver_license": "AC432223"}' -H "Content-Type: application/json" http://localhost:8081/my/path

Your body has sensitive data. Details:
 US_DRIVER_LICENSE at 41,49  
```

### Valid data request
```
curl -d '{"name":"John Smith"}' -H "Content-Type: application/json" http://localhost:8081/my/path
```
