![Build Status](https://github.com/jedisct1/iptoasn-webservice/workflows/Rust/badge.svg)

# iptoasn-webservice

Webservice to map IP addresses to AS information.

This is the source code of the public API from [iptoasn.com](https://iptoasn.com).

Requires [rust](https://www.rust-lang.org/).

# Usage:

```sh
$ curl -H'Accept: application/json' https://iptoasn-webservice.vercel.app/api/<ip address>
```

```json
{
  "announced": true,
  "as_country_code": "US",
  "as_description": "LEVEL3 - Level 3 Communications, Inc.",
  "as_number": 3356,
  "first_ip": "4.0.0.0",
  "ip": "4.3.2.1",
  "last_ip": "4.23.87.255"
}
```

### HTML Response

```sh
curl http://localhost:53661/v1/as/ip/8.8.8.8
```

Returns a formatted HTML page with the IP information.

### Unannounced IPs

For IP addresses not found in BGP announcements:

```json
{
  "announced": false,
  "ip": "127.0.0.1"
}
```

## Data Source

The service downloads and processes the IP-to-ASN mapping database from iptoasn.com, which provides comprehensive BGP routing table data updated regularly. The database is automatically cached locally and the service includes fallback mechanisms to continue operating even when the remote database is unavailable.
