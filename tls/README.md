> [!CAUTION]
> These TLS CA certificates and private keys are given as example ones to showcase SSL/TLS capabilities of the server and client. We are not generating them automatically using Certbot or something else for ease of deployment.

### An example of connecting to a TLS-enabled gRPC server in Go

```go
package main

import (
    "context"
    "crypto/x509"
    "log"
    "os"
    "time"

    "google.golang.org/grpc"
    "google.golang.org/grpc/credentials"
    "google.golang.org/grpc/examples/helloworld/helloworld"
)

func main() {
    address := "https://luna:9001"
    caCert := "./ca.pem"

    // Read the root CA and create the certificate pool to use.
    b, err := os.ReadFile(caCert)
    if err!= nil {
        log.Fatalf("error reading %s: %v", caCert, err)
    }
    pool := x509.NewCertPool()
    pool.AppendCertsFromPEM(b)

    // Connect to the remote host using our root CA.
    conn, err := grpc.Dial(address,
        grpc.WithTransportCredentials(credentials.NewClientTLSFromCert(pool, "")),
    )
    if err!= nil {
        log.Fatalf("did not connect: %v", err)
    }
    defer conn.Close()

    // Create the client and send a request.
    client := helloworld.NewGreeterClient(conn)
    ctx, cancel := context.WithTimeout(context.Background(), time.Second)
    defer cancel()
    r, err := client.SayHello(ctx, &helloworld.HelloRequest{
        Name: "Smallstep",
    })
    if err!= nil {
        log.Fatalf("could not greet: %v", err)
    }
    log.Printf("Greeting: %s", r.GetMessage())
}
```

### An example of doing the same in Rust

```rust
use tcp_chat::proto::registry_client::RegistryClient;
use tonic::transport::{Certificate, ClientTlsConfig, Channel};

const CERT: &str = include_str!("./ca.pem");

let channel = Channel::from_static("https://localhost:9001")
    .tls_config(
        ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERT))
            .domain_name("example.com"),
    )
    .unwrap()
    .connect()
    .await
    .unwrap();

let mut registry = RegistryClient::new(channel);
```
