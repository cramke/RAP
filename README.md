# Risk Aware Path Planning
Finds a path between 2 geographic locations while minimizing a risk metric.

It connects to a PostGIS database which contains vector geometries which are attributed with a risk. Another PostGIS database provides vector geometries with obstacles. These datasets are consumed by a path planning algorithm (PRM*) which generates a path that is feasible (obstruction free) and optimal (minimal risk).
