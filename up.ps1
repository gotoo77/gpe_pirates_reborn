# Update the current clean branch and build it. No cache deletion or branch switch.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
# PS 7.3+ can turn native failures into terminating errors before our explicit
# exit-code check. Keep the same checked-command behavior as Windows PowerShell.
$PSNativeCommandUseErrorActionPreference = $false
$script:UpFailureCode = 1
$step = 'initialisation'
$locationPushed = $false
$resultCode = 0

function Invoke-UpCommand {
    param([string] $Executable, [string[]] $Arguments)
    $output = & $Executable @Arguments
    $code = $LASTEXITCODE
    if ($code -ne 0) {
        $script:UpFailureCode = $code
        throw "$Executable a echoue (code $code)."
    }
    return $output
}

try {
    $step = 'localisation du projet'
    Push-Location -LiteralPath $PSScriptRoot
    $locationPushed = $true
    $projectRoot = (Get-Location).ProviderPath

    $step = 'verification du depot Git'
    $gitRoot = Invoke-UpCommand 'git' @('rev-parse', '--show-toplevel')
    if ((Resolve-Path -LiteralPath $gitRoot).ProviderPath -ne $projectRoot) {
        throw 'up.ps1 doit etre a la racine du depot Git.'
    }

    $step = 'verification des modifications locales'
    $status = @(Invoke-UpCommand 'git' @('status', '--porcelain=v1', '--untracked-files=all', '--ignore-submodules=none'))
    if ($status.Count -gt 0) {
        [Console]::Error.WriteLine(($status -join [Environment]::NewLine))
        throw 'modifications locales presentes (index, fichiers suivis ou non suivis). Aucune mise a jour.'
    }

    $step = 'verification de la branche courante'
    $branch = Invoke-UpCommand 'git' @('symbolic-ref', '--quiet', '--short', 'HEAD')
    $step = 'verification du commit (aucun commit = mise a jour indisponible)'
    $null = Invoke-UpCommand 'git' @('rev-parse', '--verify', 'HEAD')
    $step = 'verification des operations Git'
    foreach ($marker in @('MERGE_HEAD', 'CHERRY_PICK_HEAD', 'REVERT_HEAD', 'BISECT_LOG', 'rebase-merge', 'rebase-apply', 'sequencer')) {
        $markerPath = Invoke-UpCommand 'git' @('rev-parse', '--git-path', $marker)
        if (Test-Path -LiteralPath $markerPath) { throw "operation Git en cours ($marker)." }
    }

    $step = 'verification de la branche distante suivie'
    $upstream = Invoke-UpCommand 'git' @('rev-parse', '--abbrev-ref', '--symbolic-full-name', '@{upstream}')
    $null = Invoke-UpCommand 'git' @('rev-parse', '--verify', '@{upstream}')
    $step = 'verification du distant'
    $remote = Invoke-UpCommand 'git' @('config', '--get', "branch.$branch.remote")
    if ($remote -eq '.') { throw 'le suivi doit designer un distant, pas une branche locale.' }
    $null = Invoke-UpCommand 'git' @('remote', 'get-url', $remote)

    Write-Host "[up] Projet : $projectRoot"
    Write-Host "[up] Branche conservee : $branch ; suivi : $upstream"
    $step = 'mise a jour Git'
    Write-Host '[up] git pull --ff-only --no-rebase'
    Invoke-UpCommand 'git' @('-c', 'merge.autoStash=false', '-c', 'rebase.autoStash=false', 'pull', '--ff-only', '--no-rebase')

    $step = 'verification de la branche apres mise a jour Git'
    $updatedBranch = Invoke-UpCommand 'git' @('symbolic-ref', '--quiet', '--short', 'HEAD')
    if ($updatedBranch -ne $branch) { throw 'la branche courante a change pendant la mise a jour.' }
    $step = 'verification des modifications locales apres mise a jour Git'
    $updatedStatus = @(Invoke-UpCommand 'git' @('status', '--porcelain=v1', '--untracked-files=all', '--ignore-submodules=none'))
    if ($updatedStatus.Count -gt 0) { throw 'modifications locales detectees apres la mise a jour. Compilation annulee.' }

    $step = 'verification de Cargo.toml'
    if (Test-Path -LiteralPath (Join-Path $projectRoot 'Cargo.toml') -PathType Leaf) {
        $step = 'compilation Rust'
        Write-Host '[up] cargo build --locked --release'
        Invoke-UpCommand 'cargo' @('build', '--locked', '--release')
    } else {
        Write-Host '[up] Cargo.toml absent : compilation indisponible.'
    }
} catch {
    [Console]::Error.WriteLine("[up] ERREUR pendant $step : $($_.Exception.Message) Arret.")
    $resultCode = $script:UpFailureCode
} finally {
    if ($locationPushed) { Pop-Location }
}
exit $resultCode
