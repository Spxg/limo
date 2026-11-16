& {
    $ErrorActionPreference = "Stop"
    if ($env:OS -ne "Windows_NT") {
        throw "The PowerShell installer requires Windows. Use install.sh on Linux or macOS."
    }

    # Query the processor rather than the shell, which may run under emulation.
    $architecture = (Get-CimInstance Win32_Processor | Select-Object -First 1).Architecture
    $target = switch ($architecture) {
        9 { "x86_64-pc-windows-msvc" }
        12 { "aarch64-pc-windows-msvc" }
        default { throw "Unsupported Windows processor architecture: $architecture" }
    }
    $releaseBaseUrl = if ($env:LIMO_RELEASE_BASE_URL) {
        $env:LIMO_RELEASE_BASE_URL.TrimEnd("/")
    } else {
        "https://github.com/Spxg/limo/releases/latest/download"
    }
    $installDir = if ($env:LIMO_INSTALL_DIR) {
        $env:LIMO_INSTALL_DIR
    } elseif ($env:LOCALAPPDATA) {
        Join-Path $env:LOCALAPPDATA "Limo\bin"
    } else {
        throw "LOCALAPPDATA is not set. Set LIMO_INSTALL_DIR to choose an installation directory."
    }
    $installDir = [System.IO.Path]::GetFullPath($installDir)
    $assetName = "limo-$target.zip"
    $tempDir = Join-Path ([System.IO.Path]::GetTempPath()) (
        "limo-install-" + [System.Guid]::NewGuid().ToString("N")
    )
    $stagedBinary = $null
    $securityProtocol = [Net.ServicePointManager]::SecurityProtocol

    try {
        New-Item -ItemType Directory -Path $tempDir | Out-Null
        $archivePath = Join-Path $tempDir $assetName
        $extractDir = Join-Path $tempDir "package"

        Write-Host "Downloading $assetName..."
        [Net.ServicePointManager]::SecurityProtocol = $securityProtocol -bor [Net.SecurityProtocolType]::Tls12
        Invoke-WebRequest -UseBasicParsing -Uri "$releaseBaseUrl/$assetName" -OutFile $archivePath
        Expand-Archive -LiteralPath $archivePath -DestinationPath $extractDir
        $binary = Join-Path (Join-Path $extractDir "limo-$target") "limo.exe"
        if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
            throw "Downloaded archive does not contain limo-$target/limo.exe."
        }

        New-Item -ItemType Directory -Force -Path $installDir | Out-Null
        $destination = Join-Path $installDir "limo.exe"
        $stagedBinary = Join-Path $installDir (".limo-" + [System.Guid]::NewGuid().ToString("N") + ".exe")
        Copy-Item -LiteralPath $binary -Destination $stagedBinary
        if (Test-Path -LiteralPath $destination) {
            [System.IO.File]::Replace($stagedBinary, $destination, [NullString]::Value)
        } else {
            [System.IO.File]::Move($stagedBinary, $destination)
        }
        $stagedBinary = $null
        Write-Host "Installed limo to $destination"

        if ($installDir -notin ($env:PATH -split ";")) {
            Write-Host "Add $installDir to PATH, then run: limo --version"
        }
    } finally {
        [Net.ServicePointManager]::SecurityProtocol = $securityProtocol
        if ($stagedBinary -and (Test-Path -LiteralPath $stagedBinary)) {
            Remove-Item -LiteralPath $stagedBinary -Force
        }
        if (Test-Path -LiteralPath $tempDir) {
            Remove-Item -LiteralPath $tempDir -Recurse -Force
        }
    }
}
