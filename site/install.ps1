# Puts the 333 client on this machine, and does nothing else.
#
#   irm https://the333.dev/install.ps1 | iex
#   & ([scriptblock]::Create((irm https://the333.dev/install.ps1))) -Light
#
# It fetches the Windows file and the release's SHA256SUMS from github.com/needmoretruth/333,
# refuses unless the two agree, and puts the file at %LOCALAPPDATA%\Programs\333\333.exe. It
# starts nothing, installs no service or scheduled task, asks for no administrator, and changes
# your PATH only if you pass -AddToPath. Read it before you run it; it is short on purpose.
#
#   -Light       the Light form: the same client without the terminal screen (--light works too)
#   -AddToPath   add that folder to PATH for your user only (--add-to-path works too)
#
# Written for Windows PowerShell 5.1 and PowerShell 7 alike, in plain ASCII, so that either one
# reads it the same way whatever the code page.
#
# FOR TESTING ONLY: THE333_RELEASE_BASE replaces the release address, so that this script can
# be run against a directory served on this machine. Nobody installing needs it.

& {
    # Everything is inside this block so that none of it is left behind in your session when
    # the script arrives through iex, and so that a failure throws rather than closing the window.
    Set-StrictMode -Version 2.0
    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'

    # A refusal is said in the script's own lines, then stops with one short error. Thrown text
    # alone would arrive wrapped in PowerShell's account of which line of this file threw it.
    function Stop-Here([string]$text) {
        $Host.UI.WriteErrorLine($text)
        throw '333 was not installed.'
    }

    $light = $false
    $addToPath = $false
    foreach ($arg in $args) {
        switch -Regex ([string]$arg) {
            '^--?light$' { $light = $true }
            '^--?add-?to-?path$' { $addToPath = $true }
            default { Stop-Here "refused  '$arg' is not something this script understands. It takes -Light and -AddToPath." }
        }
    }

    $base = $env:THE333_RELEASE_BASE
    if ([string]::IsNullOrEmpty($base)) {
        $base = 'https://github.com/needmoretruth/333/releases/latest/download'
    }

    # Windows PowerShell 5.1 on an older .NET offers TLS 1.0 first, which GitHub refuses.
    if ($PSVersionTable.PSVersion.Major -lt 6) {
        [Net.ServicePointManager]::SecurityProtocol =
            [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
    }

    # One Windows file is built, for x86-64. Windows 11 on ARM runs it through its own
    # translation; 32-bit Windows cannot run it at all.
    $cpu = $env:PROCESSOR_ARCHITEW6432
    if ([string]::IsNullOrEmpty($cpu)) { $cpu = $env:PROCESSOR_ARCHITECTURE }
    if ($cpu -eq 'x86') {
        Stop-Here 'refused  This is 32-bit Windows, and the one Windows file is built for 64-bit x86.'
    }
    if ($light) { $form = 'Light'; $asset = '333-light-x86_64-windows.exe' }
    else { $form = 'Standard'; $asset = '333-x86_64-windows.exe' }

    if ($cpu -eq 'AMD64') { $described = 'x86-64' }
    elseif ($cpu -eq 'ARM64') { $described = '64-bit ARM' }
    else { $described = $cpu }
    Write-Host "machine  Windows on $described. Taking ${form}: $asset."
    if ($cpu -eq 'ARM64') {
        Write-Host '         That file is built for x86-64; Windows 11 on ARM translates it as it runs.'
    }

    $local = $env:LOCALAPPDATA
    if ([string]::IsNullOrEmpty($local)) {
        Stop-Here 'refused  LOCALAPPDATA is not set, so there is no Programs folder of yours to put the file in.'
    }
    $dir = Join-Path (Join-Path $local 'Programs') '333'
    $dest = Join-Path $dir '333.exe'
    $manual = "         To install it by hand instead, the commands are on https://the333.dev under`n" +
              "         'Start here', and the file itself is:`n" +
              "         $base/$asset"

    $work = Join-Path ([IO.Path]::GetTempPath()) ('333-' + [Guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $work | Out-Null
    try {
        # The sums first, so that a release without them costs one small request rather than
        # a twenty-megabyte download that is then thrown away.
        $sumsFile = Join-Path $work 'SHA256SUMS'
        try {
            Invoke-WebRequest -UseBasicParsing -Uri "$base/SHA256SUMS" -OutFile $sumsFile
        } catch {
            Stop-Here ("refused  Could not fetch SHA256SUMS from $base/SHA256SUMS.`n" +
                   "         Either this release carries none, or the network would not let it through. Without`n" +
                   "         it there is nothing to check the file against, and an unchecked file is not installed.`n" +
                   $manual)
        }

        $expected = $null
        foreach ($line in [IO.File]::ReadAllLines($sumsFile)) {
            $parts = $line.Trim() -split '\s+'
            if ($parts.Count -ge 2 -and ($parts[1] -eq $asset -or $parts[1] -eq "*$asset")) {
                $expected = $parts[0].ToLowerInvariant()
                break
            }
        }
        if ($null -eq $expected) {
            Stop-Here ("refused  SHA256SUMS in this release has no line for $asset,`n" +
                   "         so there is nothing to check it against, and it is not installed.`n" + $manual)
        }

        Write-Host "fetch    $base/$asset"
        $file = Join-Path $work $asset
        try {
            Invoke-WebRequest -UseBasicParsing -Uri "$base/$asset" -OutFile $file
        } catch {
            Stop-Here "failed   Could not fetch $base/$asset. Nothing was installed."
        }

        $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLowerInvariant()
        if ($actual -ne $expected) {
            Stop-Here ("refused  The file that arrived does not match SHA256SUMS, and it was not installed.`n" +
                   "         expected $expected`n" +
                   "         arrived  $actual`n" +
                   "         Something between here and the release changed it, or the release is broken.`n" +
                   "         It has been deleted.")
        }
        Write-Host "checked  sha256 $actual, which is what SHA256SUMS says."

        New-Item -ItemType Directory -Force -Path $dir | Out-Null
        $replacing = Test-Path -LiteralPath $dest
        try {
            Copy-Item -LiteralPath $file -Destination $dest -Force
        } catch {
            Stop-Here ("failed   Could not write $dest. If 333 is running from there, stop it first:`n" +
                   '         Windows will not replace a program while it runs. Nothing was installed.')
        }
        if ($replacing) { Write-Host "put      $dest, in place of the 333 that was there." }
        else { Write-Host "put      $dest" }
        Write-Host '         Nothing has been started, and nothing here will start on its own.'
    } finally {
        Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
    }

    # PATH, as the user's own setting says it, not as this session has it.
    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ($null -eq $userPath) { $userPath = '' }
    $onPath = @($userPath -split ';' | Where-Object { $_.TrimEnd('\') -ieq $dir }).Count -gt 0
    $run = '333'
    if ($onPath) {
        # Already there; nothing to do.
    } elseif ($addToPath) {
        $newPath = if ($userPath -eq '') { $dir } else { $userPath.TrimEnd(';') + ';' + $dir }
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
        $env:Path = ([string]$env:Path).TrimEnd(';') + ';' + $dir
        Write-Host "path     Added $dir to PATH for your user, and to this window."
        Write-Host '         To take it off again: Settings, System, About, Advanced system settings,'
        Write-Host '         Environment Variables, then Path under your user, and remove that line.'
    } else {
        $run = "& '$dest'"
        Write-Host "path     $dir is not on your PATH, so until it is, the command is the whole path."
        Write-Host '         To put it there for your user only, run this script again with -AddToPath, or'
        Write-Host '         add that folder to Path under your user in Environment Variables, and open a'
        Write-Host '         new PowerShell window.'
    }

    Write-Host "next     $run id"
    Write-Host '         That makes this machine''s name. What comes after it is on https://the333.dev'
    Write-Host "         under 'Start here'."
} @args
