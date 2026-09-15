$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$wasmDir = Join-Path $workspace "target/wasm32v1-none/release"
$bindingsDir = Join-Path $workspace "bindings"

$contracts = @(
  "consent_registry",
  "provider_registry",
  "record_attestation_registry",
  "access_receipt_registry"
)

foreach ($contract in $contracts) {
  $wasm = Join-Path $wasmDir "$contract.wasm"
  if (-not (Test-Path -LiteralPath $wasm)) {
    throw "Missing $wasm. Run 'stellar contract build' first."
  }
  stellar contract bindings typescript --wasm $wasm --output-dir (Join-Path $bindingsDir $contract) --overwrite
}
