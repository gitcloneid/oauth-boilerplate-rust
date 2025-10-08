# PowerShell script to generate self-signed certificates for HTTP/2 development
Write-Host "Generating self-signed certificates for HTTP/2 development..." -ForegroundColor Green

if (-not (Test-Path "certs")) {
    New-Item -ItemType Directory -Path "certs"
}

openssl genrsa -out certs/server.key 2048

openssl req -new -key certs/server.key -out certs/server.csr -subj "/C=ID/ST=Jakarta/L=Jakarta/O=Development/OU=IT/CN=localhost"

openssl x509 -req -days 365 -in certs/server.csr -signkey certs/server.key -out certs/server.crt

Remove-Item certs/server.csr

Write-Host "Certificates generated successfully in certs/ directory:" -ForegroundColor Green
Write-Host "  - certs/server.crt"
Write-Host "  - certs/server.key"
Write-Host ""
Write-Host "Use these with your Rust server for HTTPS + HTTP/2" -ForegroundColor Yellow
