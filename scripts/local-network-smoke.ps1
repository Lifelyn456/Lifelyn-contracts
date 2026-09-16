$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$identity = "lifelyn-ci"

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
