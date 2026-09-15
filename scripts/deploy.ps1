$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$wasmDir = Join-Path $workspace "target/wasm32v1-none/release"
$network = $env:STELLAR_NETWORK
$identity = $env:STELLAR_DEPLOYER_IDENTITY
$authority = $env:STELLAR_PROVIDER_AUTHORITY_ADDRESS
$recorder = $env:STELLAR_ACCESS_RECORDER_ADDRESS

if (-not $network -or -not $identity -or -not $authority -or -not $recorder) {
  throw "Set STELLAR_NETWORK, STELLAR_DEPLOYER_IDENTITY, STELLAR_PROVIDER_AUTHORITY_ADDRESS, and STELLAR_ACCESS_RECORDER_ADDRESS."
}
if ($identity.StartsWith("S") -or $identity.Contains(" ")) {
  throw "STELLAR_DEPLOYER_IDENTITY must be a configured Stellar CLI identity name, never a raw seed or phrase."
}

stellar contract build
$consent = stellar contract deploy --wasm (Join-Path $wasmDir "consent_registry.wasm") --source-account $identity --network $network
$provider = stellar contract deploy --wasm (Join-Path $wasmDir "provider_registry.wasm") --source-account $identity --network $network -- --authority $authority
$attestation = stellar contract deploy --wasm (Join-Path $wasmDir "record_attestation_registry.wasm") --source-account $identity --network $network
$receipt = stellar contract deploy --wasm (Join-Path $wasmDir "access_receipt_registry.wasm") --source-account $identity --network $network -- --recorder $recorder

[ordered]@{
  network = $network
  deployedAt = (Get-Date).ToUniversalTime().ToString("o")
  consentRegistry = $consent.Trim()
  providerRegistry = $provider.Trim()
  recordAttestationRegistry = $attestation.Trim()
  accessReceiptRegistry = $receipt.Trim()
} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $workspace "contract-ids.$network.json") -Encoding utf8NoBOM

& (Join-Path $PSScriptRoot "generate-bindings.ps1")
