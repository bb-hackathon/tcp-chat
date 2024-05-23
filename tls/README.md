> [!CAUTION]
> These TLS CA certificates and private keys are given as example ones to showcase SSL/TLS capabilities of the server and client. We are not generating them automatically using Certbot or something else for ease of deployment.

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
