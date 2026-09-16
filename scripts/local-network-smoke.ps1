$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$identity = "lifelyn-ci"

# `stellar container start local` returns as soon as the container process starts, but
# Stellar Core/RPC inside it need a bit longer to open their database before they can
# actually serve requests ("DB is empty" otherwise). Poll RPC's getHealth until ready.
$deadline = (Get-Date).AddSeconds(60)
$healthy = $false
do {
  Start-Sleep -Seconds 3
  try {
    $health = Invoke-RestMethod -Method Post -Uri "http://localhost:8000/rpc" `
      -ContentType "application/json" -Body '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' -ErrorAction Stop
    $healthy = $health.result.status -eq "healthy"
  } catch { $healthy = $false }
} until ($healthy -or (Get-Date) -gt $deadline)
if (-not $healthy) { throw "Local Stellar RPC did not become healthy within 60 seconds." }

stellar keys generate $identity --network local --fund --overwrite
$address = (stellar keys address $identity).Trim()
if (-not $address.StartsWith("G")) { throw "Local Stellar identity was not created." }

$env:STELLAR_NETWORK = "local"
$env:STELLAR_DEPLOYER_IDENTITY = $identity
$env:STELLAR_PROVIDER_AUTHORITY_ADDRESS = $address
$env:STELLAR_ACCESS_RECORDER_ADDRESS = $address
& (Join-Path $PSScriptRoot "deploy.ps1")

$ids = Get-Content -Raw -LiteralPath (Join-Path $workspace "contract-ids.local.json") | ConvertFrom-Json
$grantRef = "01" * 32
$subjectRef = "02" * 32
$scopeHash = "03" * 32

stellar contract invoke --id $ids.consentRegistry --source-account $identity --network local -- grant --grant_ref $grantRef --grantor $address --subject $subjectRef --recipient $address --scope_hash $scopeHash --starts_at 0 --expires_at 4102444800 | Out-Null
$active = (stellar contract invoke --id $ids.consentRegistry --source-account $identity --network local -- is_active --grant_ref $grantRef).Trim()
if ($active -ne "true") { throw "A deployed consent grant did not become active." }

stellar contract invoke --id $ids.consentRegistry --source-account $identity --network local -- revoke --grant_ref $grantRef | Out-Null
$activeAfterRevoke = (stellar contract invoke --id $ids.consentRegistry --source-account $identity --network local -- is_active --grant_ref $grantRef).Trim()
if ($activeAfterRevoke -ne "false") { throw "A deployed revoked grant remained active." }

Write-Output "Local-network consent deployment and revocation passed."
