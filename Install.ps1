$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$user = [bool]$currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
$user = [System.Convert]::ToBoolean($user) 


if ($user -eq $false) {Write-Output "You must be an administrator."; Exit} else {
    $cargo = Test-Path -Path $env:UserProfile\.cargo\bin\cargo.exe
    if ($cargo -eq $true) {
        Write-Output "Making installation."
        Write-Output "Compiling Binaries..."
        cargo build --release
        Copy-Item .\target\release\hts4.exe -Destination $env:UserProfile\.cargo\bin\hts4.exe
        $tool = Test-Path -Path $env:UserProfile\.cargo\bin\hts4.exe
        if ($tool -eq $true) {
            $hts4 = (hts4 -v)
            Write-Output $hts4, ", has been installed"
        } else {
            Write-Output "HTS4 could not be installed."
        }
    } else {
        Write-Output "Cargo is not installed."
        Exit
    }
}