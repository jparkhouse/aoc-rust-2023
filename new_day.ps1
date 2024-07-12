param (
    [Parameter(Mandatory=$true)]
    [int]$day
)

# Check if the day argument is provided
if (-not $day) {
    Write-Host "Usage: ./new_day.ps1 <day_number>"
    exit 1
}

# Create the directory for the day
$dayDir = "src/day_$day"
Write-Host "Creating directory $dayDir"
New-Item -ItemType Directory -Path $dayDir -Force

# Copy the templates to the new day directory
$templateFile = "src/mod.template.txt"
$destFile = "$dayDir/mod.rs"
Write-Host "Copying template to $destFile"
Copy-Item -Path $templateFile -Destination $destFile

# Create empty input files
$inputFiles = @("$dayDir/input.txt", "$dayDir/test_input_part_1.txt", "$dayDir/test_input_part_2.txt")
foreach ($file in $inputFiles) {
    Write-Host "Creating $file"
    New-Item -ItemType File -Path $file -Force
}

Write-Host "Generated module structure for day $day."
