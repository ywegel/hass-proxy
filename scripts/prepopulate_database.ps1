# Define the list of tuples
$tuples = @(
    "('light_paulmann', 'kitchen', 'on', '2024-03-02T12:00:00Z')",
    "('light_ikea', 'living', 'on', '2024-03-02T13:00:00Z')",
    "('light_ikea', 'living', 'off', '2024-03-02T14:00:00Z')",
    "('sensor_door', 'living', 'closed', '2024-03-02T13:00:00Z')",
    "('sensor_door', 'living', 'open', '2024-03-02T14:00:00Z')",
    "('sensor_door', 'living', 'closed', '2024-03-02T14:10:00Z')",
    "('sensor_humidity', 'kitchen', '55', '2024-03-02T14:00:00Z')",
    "('sensor_humidity', 'kitchen', '57', '2024-03-02T15:00:00Z')",
    "('sensor_humidity', 'kitchen', '58', '2024-03-02T16:00:00Z')",
    "('sensor_humidity', 'kitchen', '56', '2024-03-02T16:20:00Z')",
    "('sensor_humidity', 'bathroom', '40', '2024-03-02T14:00:00Z')",
    "('sensor_humidity', 'bathroom', '41', '2024-03-02T14:10:00Z')",
    "('sensor_humidity', 'bathroom', '43', '2024-03-02T14:30:00Z')",
    "('sensor_humidity', 'bathroom', '30', '2024-03-02T14:45:00Z')"
)

# Iterate over the tuples
foreach ($tuple in $tuples) {
    # Remove unwanted characters and split the string into elements
    $elements = $tuple -replace "[()']", "" -split ",\s*"

    # Construct the JSON object
    $json = @{
        entity_id = $elements[0]
        area = $elements[1]
        state = $elements[2]
        utc_timestamp = $elements[3]
    } | ConvertTo-Json

    Write-Output "$json`n"

    # Send the POST request
    Invoke-RestMethod -Uri "http://localhost:8080/api/entity" -Method Post -ContentType "application/json" -Body $json
}
